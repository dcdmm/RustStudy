// 第17.2节:使用 async 实现并发
// 原文:https://doc.rust-lang.org/book/ch17-02-concurrency-with-async.html
// 延续 h_async0_base.rs 的结构,按 test_01 到 test_07 的顺序阅读和运行
// 运行全部示例:cargo test -p g_concurrency --bin h_async1_concurrency -- --nocapture --test-threads=1
// 单独运行示例:cargo test -p g_concurrency --bin h_async1_concurrency test_06 -- --nocapture
// main 留空,学习示例通过测试运行
//
// 本文件沿用项目已有的 Tokio,与书中 trpl 的 API 对照如下
// trpl::block_on(...)          -> #[tokio::test] 自动创建运行时并驱动测试
// trpl::spawn_task(...)        -> tokio::spawn(...)
// trpl::sleep(...).await       -> tokio::time::sleep(...).await
// trpl::join(a, b).await       -> tokio::join!(a, b)
// trpl::join!(a, b, c)         -> tokio::join!(a, b, c)
// trpl::channel()              -> tokio::sync::mpsc::unbounded_channel()
//
// 先记住:async 构造 Future,await 等待结果,spawn 创建任务,join! 并发等待多个 Future
// 默认的 #[tokio::test] 使用单线程运行时,本文件展示单线程也能实现并发
// await 遇到尚未就绪的 Future 才会暂停当前执行,并非每次 await 都一定让出执行机会
// 不要在这里用 thread::sleep 替代异步 sleep,否则会阻塞线程,其他任务也无法推进

fn main() {}

#[cfg(test)]
mod tests {
    use std::time::Instant;
    use tokio::sync::mpsc::{self, error::TryRecvError};
    use tokio::time::{Duration, sleep};

    // 01. spawn 创建任务,JoinHandle 用于等待任务完成并获取结果
    #[tokio::test]
    async fn test_01_spawn_and_join_handle() {
        let handle = tokio::spawn(async {
            for i in 1..=3 {
                println!("子任务:第{i}次计数");
                sleep(Duration::from_millis(20)).await;
            }
            3
        });

        // spawn 已经把任务交给运行时,不需要先 await handle 才能调度它
        for i in 1..=2 {
            println!("主任务:第{i}次计数");
            sleep(Duration::from_millis(20)).await;
        }

        // handle.await 返回 Result,unwrap 取出任务正常完成时的返回值
        // 若不等待,测试返回后运行时会关闭,未完成的异步任务可能提前结束
        // 单独 drop(handle) 不会取消任务,不要把它和关闭运行时混淆
        let count = handle.await.unwrap();
        println!("已等待子任务完成,共计数{count}次");
        assert_eq!(count, 3);
    }

    // 02. join! 同时推进多个 Future,全部完成后按参数顺序返回结果元组
    #[tokio::test]
    async fn test_02_join_futures() {
        let base = 40;

        // 普通 async 块允许借用当前作用域的数据,这里不必创建独立任务
        let first = async {
            println!("A开始");
            sleep(Duration::from_millis(40)).await;
            println!("A完成");
            base + 2
        };
        let second = async {
            println!("B开始");
            sleep(Duration::from_millis(20)).await;
            println!("B完成");
            "B的结果"
        };

        // 仅创建上面的两个 Future 时,它们的代码还没有执行
        println!("两个Future已创建,现在交给join!执行");
        let (number, text) = tokio::join!(first, second);
        println!("两个Future都完成了:{number},{text}");
        assert_eq!(number, 42);
        assert_eq!(text, "B的结果");

        // 若改成 first.await; second.await;,这两个普通 Future 就会顺序执行
        // 若先 spawn 两个任务,再依次 await 它们的 handle,任务仍然可以并发执行
        // join! 的分支在当前任务中交替推进,并发不等于在多个线程上并行
        // Tokio 默认轮换优先 poll 的分支,不要依赖不同分支的固定输出顺序
        // API 说明:https://docs.rs/tokio/latest/tokio/macro.join.html
    }

    // 03. 异步 channel:发送消息不必 await,接收消息需要 await
    #[tokio::test]
    async fn test_03_async_channel() {
        // mpsc 表示 multiple producer, single consumer,即多生产者单消费者
        // 使用无界通道对应书中示例,接收端 recv 需要 &mut self,所以 rx 声明为 mut
        let (tx, mut rx) = mpsc::unbounded_channel();
        let message = String::from("hello async");

        // send 转移消息的所有权,这里之后不能再使用 message
        // 无界通道无需等待空闲容量,所以 send 返回 Result,不是 Future
        // 有界通道 mpsc::channel(capacity) 的 send 则需要 await
        tx.send(message).unwrap();
        drop(tx);

        // 即使发送端已释放,之前进入缓冲区的消息仍然可以取出
        let received = rx.recv().await;
        println!("收到:{received:?}");
        assert_eq!(received.as_deref(), Some("hello async"));

        // 所有发送端释放且缓冲区已空后,recv 返回 None
        assert_eq!(rx.recv().await, None);
        // 此例先发送后接收,虽然使用了 async,但还没有并发
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
