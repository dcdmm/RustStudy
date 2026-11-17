fn main() {}

#[cfg(test)]
mod tests {
    use std::time::Instant;
    use tokio::sync::mpsc::{self, error::TrySendError};
    use tokio::time::{Duration, sleep};

    #[tokio::test]
    async fn test01_base() {
        // Creates a bounded mpsc channel for communicating between asynchronous tasks with backpressure.
        let (tx, mut rx) = mpsc::channel(2); // 缓冲区最多存2条消息

        // 与unbounded_channel不同,send需要.await(缓冲区满时要等待空位)
        tx.send(1).await.unwrap();
        tx.send(2).await.unwrap();
        println!("发送1和2后,剩余空位:{}", tx.capacity());
        assert_eq!(tx.capacity(), 0);

        // 缓冲区已满,此时send(3)会一直等待空位
        // 这里没有其他人接收,如果直接send(3).await,程序会永远卡住

        assert_eq!(rx.recv().await, Some(1)); // 取走一条,空出一个位置
        tx.send(3).await.unwrap(); // 有空位,立即完成
        drop(tx);

        let mut received = Vec::new();
        while let Some(value) = rx.recv().await {
            received.push(value);
        }
        assert_eq!(received, [2, 3]);
    }

    #[tokio::test]
    async fn test02_backpressure() {
        // 场景: 发送很快,接收很慢(每条消息处理20毫秒)
        // 如果用unbounded_channel,发送端会一口气发完,消息全部堆积在缓冲区
        // 用有界通道,缓冲区满后send等待,发送端被迫放慢到接收端的速度
        // 这就是背压(backpressure): 下游处理不过来时,让上游慢下来
        let (tx, mut rx) = mpsc::channel(1);
        let start = Instant::now();

        let sender = async move {
            for value in 1..=4 {
                tx.send(value).await.unwrap(); // 缓冲区满时在此等待
                println!("发送{value},经过{}毫秒", start.elapsed().as_millis());
            }
            start.elapsed()
        };
        let receiver = async {
            while let Some(value) = rx.recv().await {
                println!("接收{value},经过{}毫秒", start.elapsed().as_millis());
                sleep(Duration::from_millis(20)).await; // 模拟处理耗时
            }
        };

        let (send_time, ()) = tokio::join!(sender, receiver);
        // 接收端每取走一条,发送端才能再发一条,4条消息至少要40毫秒才能发完
        assert!(send_time >= Duration::from_millis(40));
    }

    #[tokio::test]
    async fn test03_try_send() {
        let (tx, mut rx) = mpsc::channel(1);

        // try_send不等待,立即返回结果,适合"发不出去就算了"的场景(如: 日志)
        assert_eq!(tx.try_send(1), Ok(()));

        // 缓冲区已满,返回Full,没发出去的值会还回来
        assert_eq!(tx.try_send(2), Err(TrySendError::Full(2)));

        assert_eq!(rx.recv().await, Some(1)); // 取走一条
        assert_eq!(tx.try_send(2), Ok(())); // 有空位,发送成功
    }
}
