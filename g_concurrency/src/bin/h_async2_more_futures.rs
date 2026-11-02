fn main() {}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::time::Instant;
    use tokio::runtime::Runtime;
    use tokio::sync::mpsc;
    use tokio::time::{Duration, sleep};

    fn slow(name: &str, ms: u64) {
        std::thread::sleep(Duration::from_millis(ms));
        println!("{name}执行了{ms}毫秒的阻塞工作");
    }

    #[tokio::test]
    async fn test01_blocking_without_yield() {
        let events = RefCell::new(Vec::new());
        let a = async {
            for step in ["A1", "A2", "A3"] {
                // std::thread::sleep会阻塞整个线程
                slow(step, 10);
                events.borrow_mut().push(step);
            }
            // tokio的sleep不阻塞线程,a在此暂停,b这时才开始执行
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
    async fn test02_blocking_with_yield() {
        let events = RefCell::new(Vec::new());
        let a = async {
            for step in ["A1", "A2", "A3"] {
                slow(step, 10);
                events.borrow_mut().push(step);
                // a每步都在此暂停,A1之后b就开始执行
                sleep(Duration::from_millis(10)).await;
            }
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
        assert_eq!(events, ["A1", "B开始", "A2", "A3"]);
    }

    #[tokio::test]
    async fn test03_await_may_not_yield() {
        let events = RefCell::new(Vec::new());
        let (tx, mut rx) = mpsc::unbounded_channel();
        // 消息提前放进队列
        tx.send(42).unwrap();
        let a = async {
            events.borrow_mut().push("A1");
            // 消息已在队列中,recv立即返回,a不会暂停
            let value = rx.recv().await.unwrap();
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
    async fn test04_select_race_cancel() {
        let (tx, mut rx) = mpsc::unbounded_channel::<&str>();
        let slow_future = async move {
            println!("慢Future开始,持有发送端");
            sleep(Duration::from_secs(10)).await;
            tx.send("慢Future完成").unwrap();
            "慢Future"
        };
        let fast_future = async {
            sleep(Duration::from_millis(10)).await;
            "快Future"
        };

        // join!等全部Future完成,select!只要一个完成就返回,其余的被取消
        let winner = tokio::select! {
            biased;
            value = slow_future => value,
            value = fast_future => value,
        };
        println!("获胜者:{winner}");
        assert_eq!(winner, "快Future");
        // slow_future被取消时tx随之drop,channel关闭,recv返回None
        assert_eq!(rx.recv().await, None);
    }

    #[tokio::test]
    async fn test05_timeout_expiry() {
        // 模拟一个卡住的网络请求(需要10s)
        let request = async {
            sleep(Duration::from_secs(10)).await;
            "响应数据"
        };

        // 最多等待20毫秒: 按时完成返回Ok,超时返回Err并取消request
        let result = tokio::time::timeout(Duration::from_millis(20), request).await;

        let message = match result {
            Ok(data) => format!("收到响应:{data}"),
            Err(_) => String::from("请求超时"),
        };
        println!("{message}");
        assert_eq!(message, "请求超时");
    }

    #[tokio::test]
    async fn test06_no_preemption() {
        let work = async {
            // slow让整个线程停住3秒,timeout也在这个线程上,这3秒内无法检查时间
            slow("阻塞工作", 3000);
            // 工作在此暂停,timeout才有机会检查时间,发现已超时就返回Err
            // 去掉这一行,工作会直接完成,timeout不检查时间,返回Ok
            sleep(Duration::from_millis(2)).await;
            "阻塞工作完成"
        };
        let start = Instant::now();
        let result = tokio::time::timeout(Duration::from_millis(10), work).await;

        println!("限制1毫秒,实际耗时:{:?},结果:{result:?}", start.elapsed());
        assert_eq!(result, Ok("阻塞工作完成"));
    }

    #[test]
    fn test07_block_on() {
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .expect("创建运行时失败");
        fn get_number(runtime: &Runtime) -> u32 {
            let future = async {
                sleep(Duration::from_millis(10)).await;
                42
            };

            // 普通函数运行异步代码,block_on会阻塞当前线程,直到future执行完
            runtime.block_on(future)
        }

        let result = get_number(&runtime);
        println!("{result}");
        assert_eq!(result, 42);
    }
}
