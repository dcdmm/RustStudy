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

        // 缓冲区已满,send(3)会一直等待空位
        // tx.send(3).await.unwrap();

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
        // 背压(backpressure): 接收慢时缓冲区会满,send只能等待,发送端被迫跟着变慢
        // 若用unbounded_channel,发送端会一口气发完,消息全部堆积在缓冲区
        let (tx, mut rx) = mpsc::channel(2);
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
        // 1和2放入缓冲区,接收端立即取走1,3也不用等,只有4要等接收端处理完一条(20毫秒),所以发送端发完4条至少需要20毫秒
        assert!(send_time >= Duration::from_millis(20));
    }

    #[tokio::test]
    async fn test03_try_send() {
        let (tx, mut rx) = mpsc::channel(1);

        // try_send不需要.await(有空位就发送,没空位立即返回错误,不会等待)
        assert_eq!(tx.try_send(1), Ok(()));

        // 缓冲区已满,返回Full(2),未发送的值通过错误返回,便于重试或丢弃
        assert_eq!(tx.try_send(2), Err(TrySendError::Full(2)));

        assert_eq!(rx.recv().await, Some(1));
        assert_eq!(tx.try_send(2), Ok(()));
    }
}
