fn main() {}

#[cfg(test)]
mod tests {
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
        // Semaphore(信号量): 许可数量固定,拿到许可才能执行,许可用完时需等待其他任务归还
        let semaphore = Arc::new(Semaphore::new(2)); // 最多同时执行2个任务,用Arc让所有任务共同拥有同一个信号量
        let start = Instant::now();

        let mut set = JoinSet::new();
        for id in 1..=6 {
            let semaphore = Arc::clone(&semaphore);
            set.spawn(async move {
                let _permit = semaphore.acquire().await.unwrap();
                request(id, start).await
            }); // 任务结束时_permit被drop,许可归还
        }
        let ids = set.join_all().await;

        println!("最多同时2个,总耗时{}毫秒", start.elapsed().as_millis());
    }
}
