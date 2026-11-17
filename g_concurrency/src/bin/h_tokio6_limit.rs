fn main() {}

#[cfg(test)]
mod tests {
    use futures::stream::{self, StreamExt};
    use std::sync::Arc;
    use std::time::Instant;
    use tokio::sync::Semaphore;
    use tokio::task::JoinSet;
    use tokio::time::{Duration, sleep};

    // 模拟一次耗时50毫秒的请求
    async fn request(id: u32, start: Instant) -> u32 {
        println!("请求{id}开始,经过{}毫秒", start.elapsed().as_millis());
        sleep(Duration::from_millis(50)).await;
        id
    }

    #[tokio::test]
    async fn test01_semaphore() {
        // 不加限制时,所有请求会同时开始
        // 请求很多时(如: 10000个),会同时打开大量连接,压垮对方服务器或耗尽本机资源

        // Counting semaphore performing asynchronous permit acquisition.
        // Semaphore(信号量): 一共有N个许可(permit),拿到许可才能执行,用完归还
        // 许可被拿光时,acquire在此等待,直到有人归还
        let semaphore = Arc::new(Semaphore::new(2)); // 最多同时2个,用Arc在多个任务间共享
        let start = Instant::now();

        let mut set = JoinSet::new();
        for id in 1..=6 {
            let semaphore = Arc::clone(&semaphore);
            set.spawn(async move {
                // 注意是_permit而不是_,let _ = ...会立即drop,许可马上归还,起不到限制作用
                let _permit = semaphore.acquire().await.unwrap();
                request(id, start).await
            }); // 任务结束时_permit被drop,许可归还
        }
        let ids = set.join_all().await; // 等待全部任务完成,收集结果

        println!("最多同时2个,总耗时{}毫秒", start.elapsed().as_millis());
        assert_eq!(ids.len(), 6);
        // 6个请求分3批执行,每批50毫秒
        assert!(start.elapsed() >= Duration::from_millis(150));
    }

    #[tokio::test]
    async fn test02_buffer_unordered() {
        // 不需要spawn时的更简单写法(futures库): 把一批Future放进Stream,最多同时执行n个
        // 这些Future在当前任务中并发执行(类似join!),不会创建新任务
        let start = Instant::now();
        let ids: Vec<u32> = stream::iter(1..=6)
            .map(|id| request(id, start)) // 每个id变成一个Future,此时还没开始执行
            .buffer_unordered(2) // 最多同时执行2个,谁先完成先返回谁(需要保持原顺序时用buffered)
            .collect()
            .await;

        println!("buffer_unordered结果:{ids:?},总耗时{}毫秒", start.elapsed().as_millis());
        assert_eq!(ids.len(), 6);
        assert!(start.elapsed() >= Duration::from_millis(150));
    }
}
