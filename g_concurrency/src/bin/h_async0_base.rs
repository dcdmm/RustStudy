fn main() {}

#[cfg(test)]
mod tests {
    use std::time::Instant;
    use tokio::runtime::Runtime;
    use tokio::time::{Duration, sleep};

    #[tokio::test]
    async fn test_01_base() {
        // 异步函数
        async fn answer() -> u32 {
            println!("异步函数开始执行");
            42
        }

        let future = answer();

        let value = future.await;
        println!("异步函数结果:{value}");
        assert_eq!(value, 42);

        let base = 40;
        // 异步代码块
        let future_block = async { base + 2 };
        let value_block = future_block.await;
        println!("异步代码块结果:{value_block}");
        assert_eq!(value_block, 42);
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
        // 普通函数顺序执行:先完成 A,再开始 B
        fn sequential() {
            work_sync("A");
            work_sync("B");
        }

        // 异步函数顺序执行:逐个.await,先完成 A,再开始 B
        async fn async_sequential() {
            work("A").await;
            work("B").await;
        }

        // 异步并发执行:等待 A 时可以推进 B,两者都完成才继续
        async fn concurrent() {
            tokio::join!(work("A"), work("B"));
        }

        let sequential_start = Instant::now();
        sequential();
        println!(
            "sequential运行时间:{:.3}毫秒",
            sequential_start.elapsed().as_secs_f64() * 1000.0
        );

        let async_sequential_start = Instant::now();
        async_sequential().await;
        println!(
            "async_sequential运行时间:{:.3}毫秒",
            async_sequential_start.elapsed().as_secs_f64() * 1000.0
        );

        let concurrent_start = Instant::now();
        concurrent().await;
        println!(
            "concurrent运行时间:{:.3}毫秒",
            concurrent_start.elapsed().as_secs_f64() * 1000.0
        );
    }

    #[tokio::test]
    async fn test_03_await_chain() {
        struct Number(u32);

        impl Number {
            async fn double(self) -> u32 {
                self.0 * 2
            }
        }

        async fn get_number() -> Number {
            Number(21)
        }

        // 调式调用
        let chained_result = get_number().await.double().await;
        println!("链式调用结果:{chained_result}");
    }

    #[test]
    fn test_04_runtime() {
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .expect("创建运行时失败");
        fn get_number(runtime: &Runtime) -> u32 {
            let future = async {
                sleep(Duration::from_millis(10)).await;
                42
            };

            // 普通函数运行异步代码
            runtime.block_on(future)
        }

        let result = get_number(&runtime);
        println!("{result}");
    }

    #[tokio::test]
    async fn test_05_pause_and_resume() {
        let task_a = async {
            let mut value = 40;
            println!("A:保存value={value},开始等待");
            sleep(Duration::from_millis(20)).await;
            value += 2;
            println!("A:恢复执行,value={value}");
            value
        };
        let task_b = async {
            println!("B:在A等待期间执行");
        };

        let (result, ()) = tokio::join!(task_a, task_b);
        assert_eq!(result, 42);
    }

    #[tokio::test]
    async fn test_06_select() {
        let winner = tokio::select! {
            _ = work("A") => "A",
            _ = work("B") => "B",
        };

        println!("先完成的是:{winner}");
    }
}
