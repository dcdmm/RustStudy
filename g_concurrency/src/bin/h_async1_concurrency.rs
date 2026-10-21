fn main() {}

#[cfg(test)]
mod tests {
    use std::time::Instant;
    use tokio::sync::mpsc::{self, error::TryRecvError};
    use tokio::time::{Duration, sleep};

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
    async fn test01_spawn() {
        let start = Instant::now();

        let handle_a = tokio::spawn(work("A"));
        let handle_b = tokio::spawn(work("B"));

        handle_a.await.unwrap();
        handle_b.await.unwrap();

        println!(
            "spawn运行时间:{:.3}毫秒",
            start.elapsed().as_secs_f64() * 1000.0
        );
    }

    #[tokio::test]
    async fn test02_join() {
        // 普通函数顺序执行:先完成A,再开始B
        fn sequential() {
            work_sync("A");
            work_sync("B");
        }

        // 异步函数顺序执行:逐个.await,先完成A,再开始B
        async fn async_sequential() {
            work("A").await;
            work("B").await;
        }

        // 异步并发执行:等待A时可以推进B,两者都完成才继续
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
    async fn test_03_async_channel() {
        let (tx, mut rx) = mpsc::unbounded_channel();
        let message = String::from("hello async");

        tx.send(message).unwrap();
        drop(tx);

        let received = rx.recv().await;
        println!("收到:{received:?}");

        assert_eq!(rx.recv().await, None);
    }

    // 04. 常见误区:同一个 async 块内先发送再接收,仍然是顺序执行
    #[tokio::test]
    async fn test_04_one_async_block_is_sequential() {
        let (tx, mut rx) = mpsc::unbounded_channel();
        let start = Instant::now();

        for value in ["hi", "from", "async"] {
            tx.send(value).unwrap();
            println!("发送:{value},经过{}毫秒", start.elapsed().as_millis());
            sleep(Duration::from_millis(20)).await;
        }
        // 显式释放发送端,让这个反例也能正常退出
        drop(tx);

        let mut received = Vec::new();
        while let Some(value) = rx.recv().await {
            println!("接收:{value},经过{}毫秒", start.elapsed().as_millis());
            received.push(value);
        }
        assert_eq!(received, ["hi", "from", "async"]);

        // 观察输出:等待约60毫秒后,才集中打印三条接收消息
        // sleep.await 只暂停当前执行,不会让代码跳过 for 循环去执行下面的 while
        // 时间仅用于观察,不使用精确耗时断言,避免机器调度影响测试结果
    }

    // 05. 拆成两个 Future 后可以并发,但借用 tx 不会让它随发送 Future 一起释放
    #[tokio::test]
    async fn test_05_borrowed_sender_keeps_channel_open() {
        let (tx, mut rx) = mpsc::unbounded_channel();

        // send 只需要 &self,因此这个 async 块只会借用外层的 tx
        let sender = async {
            for value in [10, 20] {
                tx.send(value).unwrap();
                sleep(Duration::from_millis(20)).await;
            }
        };
        let receiver = async {
            let mut received = Vec::new();
            // 为了安全观察这个问题,这里只接收已知的两条消息
            for _ in 0..2 {
                let value = rx.recv().await.unwrap();
                println!("边发送边接收:{value}");
                received.push(value);
            }
            received
        };

        let ((), received) = tokio::join!(sender, receiver);
        assert_eq!(received, [10, 20]);

        // Empty 表示当前没有消息,但发送端仍然存在,此时 recv().await 会继续等待
        assert_eq!(rx.try_recv(), Err(TryRecvError::Empty));
        println!("发送Future已完成,但外层tx还存在,通道仍未关闭");

        // 若 receiver 改为 while let Some(...),join! 就会一直等待
        // 循环依赖关系:join! 等接收结束 -> 接收等 tx 释放 -> tx 等外层作用域结束
        drop(tx);
        assert_eq!(rx.recv().await, None);
    }

    // 06. async move 转移 tx 的所有权,发送结束后释放它,让接收循环自然退出
    #[tokio::test]
    async fn test_06_async_move_closes_channel() {
        let (tx, mut rx) = mpsc::unbounded_channel();
        let start = Instant::now();

        // 与上一个示例相比,关键变化是增加 move
        // move 改变捕获数据的所有权,不会创建线程或启动任务
        let sender = async move {
            for value in ["hi", "from", "async"] {
                tx.send(value).unwrap();
                sleep(Duration::from_millis(20)).await;
            }
            // tx 属于这个 async 块,执行完后自动释放
        };
        let receiver = async {
            let mut received = Vec::new();
            while let Some(value) = rx.recv().await {
                println!("接收:{value},经过{}毫秒", start.elapsed().as_millis());
                received.push(value);
            }
            println!("发送端已释放且消息已取完,接收循环结束");
            received
        };

        let ((), received) = tokio::join!(sender, receiver);
        assert_eq!(received, ["hi", "from", "async"]);
        // 对比 test_04:现在接收消息之间能观察到等待间隔
        // 对比 test_05:现在可以持续接收直到 None,无需提前知道消息数量
    }

    // 07. clone 发送端支持多个生产者,join! 可以组合编译期已知数量的 Future
    #[tokio::test]
    async fn test_07_multiple_producers() {
        let (tx, mut rx) = mpsc::unbounded_channel();
        let tx1 = tx.clone();

        // 克隆的是同一个通道的发送端,不是创建第二个通道
        let sender_a = async move {
            for value in ["A1", "A2", "A3"] {
                tx1.send(value).unwrap();
                sleep(Duration::from_millis(20)).await;
            }
        };
        // 接收 Future 放在 sender_b 前面定义也没有关系,创建顺序不决定执行顺序
        let receiver = async {
            let mut received = Vec::new();
            while let Some(value) = rx.recv().await {
                println!("收到多个生产者的消息:{value}");
                received.push(value);
            }
            received
        };
        let sender_b = async move {
            for value in ["B1", "B2"] {
                tx.send(value).unwrap();
                sleep(Duration::from_millis(35)).await;
            }
        };

        // 两个发送块都使用 async move,原始 tx 和克隆的 tx1 都会在各自完成后释放
        // 只释放一个发送端不够,若外层还保留其他克隆,接收循环仍然无法自然结束
        // join! 内部已经执行等待,宏调用后不再加 .await
        let ((), (), received) = tokio::join!(sender_a, sender_b, receiver);

        assert_eq!(received.len(), 5);
        // 同一生产者的消息保持发送顺序,不同生产者之间的交错顺序不作保证
        let from_a: Vec<_> = received
            .iter()
            .copied()
            .filter(|v| v.starts_with('A'))
            .collect();
        let from_b: Vec<_> = received
            .iter()
            .copied()
            .filter(|v| v.starts_with('B'))
            .collect();
        assert_eq!(from_a, ["A1", "A2", "A3"]);
        assert_eq!(from_b, ["B1", "B2"]);
    }
}
