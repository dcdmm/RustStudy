fn main() {}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::future::{Future, pending, ready};
    use std::time::Instant;
    use tokio::sync::mpsc;
    use tokio::task::yield_now;
    use tokio::time::{Duration, sleep};

    fn slow(name: &str, ms: u64) {
        std::thread::sleep(Duration::from_millis(ms));
        println!("{name}执行了{ms}毫秒的阻塞工作");
    }

    async fn timeout<F: Future>(
        future_to_try: F,
        max_time: Duration,
    ) -> Result<F::Output, Duration> {
        tokio::select! {
            biased;
            output = future_to_try => Ok(output),
            _ = sleep(max_time) => Err(max_time),
        }
    }

    #[tokio::test]
    async fn test_01_blocking_work_delays_other_futures() {
        let events = RefCell::new(Vec::new());
        let a = async {
            for step in ["A1", "A2", "A3"] {
                slow(step, 10);
                events.borrow_mut().push(step);
            }
            sleep(Duration::from_millis(10)).await;
            println!("A完成");
        };
        let b = async {
            events.borrow_mut().push("B开始");
            slow("B", 10);
            println!("B完成");
        };

        tokio::join!(biased; a, b);
        let events = events.into_inner();
        println!("执行顺序:{events:?}");
        assert_eq!(events, ["A1", "A2", "A3", "B开始"]);
    }

    #[tokio::test]
    async fn test_02_ready_await_does_not_yield() {
        let events = RefCell::new(Vec::new());
        let a = async {
            events.borrow_mut().push("A1");
            // ready(42) 把 42 包装成已经就绪的 Future,.await 立即取出结果,不会暂停
            // 这里用于演示:有 .await 不代表一定让出执行机会
            let value = ready(42).await;
            events.borrow_mut().push("A2");
            value
        };
        let b = async {
            events.borrow_mut().push("B");
        };

        let (value, ()) = tokio::join!(biased; a, b);
        let events = events.into_inner();
        println!("虽然A中有.await,执行顺序仍然是:{events:?}");
        assert_eq!(value, 42);
        assert_eq!(events, ["A1", "A2", "B"]);
    }

    #[tokio::test]
    async fn test_03_sleep_allows_interleaving() {
        let a = async {
            for step in ["A1", "A2", "A3"] {
                slow(step, 5);
                // 等计时器期间,join!可以推进另一个Future
                sleep(Duration::from_millis(10)).await;
            }
            "A完成"
        };
        let b = async {
            for step in ["B1", "B2", "B3"] {
                slow(step, 5);
                sleep(Duration::from_millis(10)).await;
            }
            "B完成"
        };

        let result = tokio::join!(a, b);
        println!("两个Future都完成:{result:?}");
        assert_eq!(result, ("A完成", "B完成"));
    }

    #[tokio::test]
    async fn test_04_yield_now_without_a_timer() {
        let a = async {
            for step in ["A1", "A2", "A3"] {
                slow(step, 5);
                // 只想让出执行机会,无需人为等待某个时间长度
                yield_now().await;
            }
            "A完成"
        };
        let b = async {
            for step in ["B1", "B2", "B3"] {
                slow(step, 5);
                yield_now().await;
            }
            "B完成"
        };

        // 这就是协作式多任务:主动在合适的位置让出控制权
        // yield_now 不保证下一次一定运行另一个任务,不要依赖严格交替的输出
        // slow 本身仍然阻塞,让出机会只发生在每段工作之后
        let result = tokio::join!(a, b);
        assert_eq!(result, ("A完成", "B完成"));
    }

    #[tokio::test]
    async fn test_05_compare_sleep_and_yield_cost() {
        let start = Instant::now();
        for _ in 0..10 {
            // 纳秒参数不代表计时器具备纳秒精度,实际等待可能长得多
            sleep(Duration::from_nanos(1)).await;
        }
        let sleep_elapsed = start.elapsed();

        let start = Instant::now();
        for _ in 0..10 {
            yield_now().await;
        }
        let yield_elapsed = start.elapsed();

        println!("10次sleep(1纳秒):{sleep_elapsed:?}");
        println!("10次yield_now:{yield_elapsed:?}");
        // 仅观察开销,不断言谁一定更快,耗时会受操作系统调度影响
        // 让出控制权也有成本,应结合测量决定工作分块的大小和让出频率
    }

    #[tokio::test]
    async fn test_06_select_returns_after_one_finishes() {
        let (tx, mut rx) = mpsc::unbounded_channel::<&str>();
        let slow_future = async move {
            println!("慢Future开始,持有发送端");
            // 用永不就绪的 Future 模拟一直等不到结果,避免依赖速度差判断胜者
            pending::<()>().await;
            tx.send("慢Future完成").unwrap();
            "慢Future"
        };
        let fast_future = async {
            sleep(Duration::from_millis(10)).await;
            "快Future"
        };

        // join! 等全部完成,select! 在一个分支完成后就返回
        let winner = tokio::select! {
            biased;
            value = slow_future => value,
            value = fast_future => value,
        };
        println!("获胜者:{winner}");
        assert_eq!(winner, "快Future");

        assert_eq!(rx.recv().await, None);
    }

    #[tokio::test]
    async fn test_07_biased_select_checks_in_order() {
        // Tokio 的 select! 默认随机选择先检查的分支
        // biased 则从上往下检查,对应书中 trpl::select 的优先顺序
        let first = tokio::select! {
            biased;
            value = ready("A") => value,
            value = ready("B") => value,
        };
        let reversed = tokio::select! {
            biased;
            value = ready("B") => value,
            value = ready("A") => value,
        };

        println!("都已就绪时,先检查的分支获胜:{first},{reversed}");
        assert_eq!(first, "A");
        assert_eq!(reversed, "B");
    }

    #[tokio::test]
    async fn test_08_timeout_returns_the_future_output() {
        let work = async {
            yield_now().await;
            "工作完成"
        };
        let result = timeout(work, Duration::from_secs(1)).await;
        println!("限时内完成:{result:?}");
        assert_eq!(result, Ok("工作完成"));

        // 同一个泛型函数也能返回数字,无需为不同输出类型分别实现
        // 即使限制为零,已经就绪的工作也会因为优先检查而成功
        let number = timeout(ready(42), Duration::ZERO).await;
        assert_eq!(number, Ok(42));
    }

    #[tokio::test]
    async fn test_09_timeout_returns_an_error() {
        let max_time = Duration::from_millis(20);
        // 工作永远不会就绪,计时器就绪后返回 Err,不会一直等待
        let result = timeout(pending::<String>(), max_time).await;

        match &result {
            Ok(message) => println!("成功:{message}"),
            Err(duration) => println!("超时,时间限制为{}毫秒", duration.as_millis()),
        }
        assert_eq!(result, Err(max_time));
        // Err 保存的是传入的时间限制,不表示实际耗时精确等于这个值
    }

    #[tokio::test]
    async fn test_10_timeout_cannot_interrupt_blocking_work() {
        // 补充边界:计时器也是 Future,必须有机会被轮询才能报告超时
        let work = async {
            slow("阻塞工作", 3000);
            "阻塞工作完成"
        };
        let start = Instant::now();
        let result = timeout(work, Duration::from_millis(1)).await;

        println!("限制1毫秒,实际耗时:{:?},结果:{result:?}", start.elapsed());
        // 自制 timeout 优先轮询工作,工作没有暂停,直接阻塞到完成并返回 Ok
        // async 不会把同步代码自动变成可中断的工作
        assert_eq!(result, Ok("阻塞工作完成"));
    }
}
