fn main() {}

#[cfg(test)]
mod tests {
    use std::time::Instant;
    use tokio::time::{Duration, sleep};

    #[tokio::test]
    async fn test_01_func_block() {
        // 异步函数
        async fn answer() -> u32 {
            println!("2. 异步函数开始执行");
            42
        }

        let future = answer();

        let value = future.await;
        println!("异步函数结果:{value}");

        let base = 40;
        // 异步代码块
        let future_block = async { base + 2 };
        let value_block = future_block.await;
        println!("异步代码块结果:{value_block}");
    }

    // 普通函数:阻塞当前线程200毫秒
    fn work_sync(name: &str) {
        println!("{name}开始");
        std::thread::sleep(Duration::from_millis(200));
        println!("{name}完成");
    }

    // 异步函数:等待200毫秒期间可以推进其他异步操作
    async fn work(name: &str) {
        println!("{name}开始");
        sleep(Duration::from_millis(200)).await;
        println!("{name}完成");
    }

    #[tokio::test]
    async fn test_02_compare() {
        // 普通函数顺序执行:先完成A,再开始 B
        fn sequential() {
            work_sync("A");
            work_sync("B");
        }
        // 异步并发执行:等待 A 时可以推进 B,两者都完成才继续
        async fn parallel() {
            tokio::join!(work("A"), work("B"));
        }

        let sequential_start = Instant::now();
        sequential();
        println!(
            "sequential运行时间:{:.3}毫秒",
            sequential_start.elapsed().as_secs_f64() * 1000.0
        );

        let parallel_start = Instant::now();
        parallel().await;
        println!(
            "parallel运行时间:{:.3}毫秒",
            parallel_start.elapsed().as_secs_f64() * 1000.0
        );
    }

    // 5. 竞速:只取先完成的分支,本例不规定 A 或 B 必须获胜
    #[tokio::test]
    async fn test_05_select() {
        let winner = tokio::select! {
            _ = work("A") => "A",
            _ = work("B") => "B",
        };

        // 本例未完成的 Future 会被丢弃,不会继续打印完成
        println!("先完成的是:{winner}");
    }
}
