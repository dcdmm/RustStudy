fn main() {}

#[cfg(test)]
mod tests {
    use std::time::Instant;
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

    #[tokio::test]
    async fn test_02_await_chain() {
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

    // 普通: 阻塞当前线程约200毫秒
    fn work_sync(name: &str) {
        println!("{name}开始");
        std::thread::sleep(Duration::from_millis(200));
        println!("{name}完成");
    }

    // 异步: 等待定时器时不阻塞当前线程,可通过join!/spawn与其他异步操作并发
    async fn work(name: &str) {
        println!("{name}开始");
        sleep(Duration::from_millis(200)).await;
        println!("{name}完成");
    }

    #[tokio::test]
    async fn test03_spawn_and_join() {
        // 普通函数顺序执行: 先完成A,再开始B,最后完成C
        fn sequential() {
            work_sync("A");
            work_sync("B");
            work_sync("C");
        }

        // 异步函数顺序执行: 逐个.await,先完成A,再开始B,最后完成C
        async fn async_sequential() {
            work("A").await;
            work("B").await;
            work("C").await;
        }

        // Python版本
        // import asyncio
        //
        // async def work(name: str):
        //     print(f"{name}开始")
        //     await asyncio.sleep(0.2)
        //     print(f"{name}完成")
        //
        // async def concurrent_join():
        //     await asyncio.gather(work("A"), work("B"))
        //
        // async def concurrent_spawn():
        //     handle_a = asyncio.create_task(work("A"))
        //     handle_b = asyncio.create_task(work("B"))
        //
        //     await handle_a
        //     await handle_b

        async fn concurrent_join() {
            tokio::join!(work("A"), work("B"));

            println!("执行后续代码");

            work("C").await;
        }

        async fn concurrent_spawn() {
            let handle_a = tokio::spawn(work("A"));
            let handle_b = tokio::spawn(work("B"));
            let handle_c = tokio::spawn(work("B"));

            println!("执行后续代码");

            handle_a.await.unwrap();
            handle_b.await.unwrap();
            handle_c.await.unwrap();
        }

        let sequential_start = Instant::now();
        sequential();
        println!(
            "sequential:{:.3}毫秒",
            sequential_start.elapsed().as_secs_f64() * 1000.0
        );

        let async_sequential_start = Instant::now();
        async_sequential().await;
        println!(
            "async_sequential运行时间:{:.3}毫秒",
            async_sequential_start.elapsed().as_secs_f64() * 1000.0
        );

        let join_start = Instant::now();
        concurrent_join().await;
        println!(
            "join运行时间:{:.3}毫秒",
            join_start.elapsed().as_secs_f64() * 1000.0
        );

        let spawn_start = Instant::now();
        concurrent_spawn().await;
        println!(
            "spawn运行时间:{:.3}毫秒",
            spawn_start.elapsed().as_secs_f64() * 1000.0
        );
    }
}
