fn main() {}

#[cfg(test)]
mod tests {
    use std::time::Instant;
    use tokio::sync::mpsc::{self, error::TryRecvError};
    use tokio::time::{Duration, sleep};

    #[tokio::test]
    async fn test03_async_channel() {
        let (tx, mut rx) = mpsc::unbounded_channel();
        let message = String::from("hello async");

        tx.send(message).unwrap();
        drop(tx);

        let received = rx.recv().await;
        println!("收到:{received:?}");

        assert_eq!(rx.recv().await, None);
    }

    #[tokio::test]
    async fn test_04_one_async_block_is_sequential() {
        let (tx, mut rx) = mpsc::unbounded_channel();
        let start = Instant::now();

        for value in ["hi", "from", "async"] {
            tx.send(value).unwrap();
            println!("发送:{value},经过{}毫秒", start.elapsed().as_millis());
            sleep(Duration::from_millis(20)).await;
        }

        drop(tx);

        let mut received = Vec::new();
        while let Some(value) = rx.recv().await {
            println!("接收:{value},经过{}毫秒", start.elapsed().as_millis());
            received.push(value);
        }
        assert_eq!(received, ["hi", "from", "async"]);
    }

    #[tokio::test]
    async fn test_05_borrowed_sender_keeps_channel_open() {
        let (tx, mut rx) = mpsc::unbounded_channel();

        let sender = async {
            for value in [10, 20, 30] {
                tx.send(value).unwrap();
                println!("发送{value}");
                sleep(Duration::from_millis(20)).await;
            }
        };
        let receiver = async {
            let mut received = Vec::new();
            for _ in 0..3 {
                let value = rx.recv().await.unwrap();
                println!("接收:{value}");
                received.push(value);
            }
            received
        };

        let ((), received) = tokio::join!(sender, receiver);
        assert_eq!(received, [10, 20, 30]);

        assert_eq!(rx.try_recv(), Err(TryRecvError::Empty));
        println!("发送Future已完成,但外层tx还存在,通道仍未关闭");

        drop(tx);
        assert_eq!(rx.recv().await, None);
    }

    #[tokio::test]
    async fn test_06_async_move_closes_channel() {
        let (tx, mut rx) = mpsc::unbounded_channel();
        let start = Instant::now();

        let sender = async move {
            for value in ["hi", "from", "async"] {
                tx.send(value).unwrap();
                sleep(Duration::from_millis(20)).await;
            }
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
    }

    #[tokio::test]
    async fn test_07_multiple_producers() {
        let (tx, mut rx) = mpsc::unbounded_channel();
        let tx1 = tx.clone();

        let sender_a = async move {
            for value in ["A1", "A2", "A3"] {
                tx1.send(value).unwrap();
                sleep(Duration::from_millis(20)).await;
            }
        };

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

        let ((), (), received) = tokio::join!(sender_a, sender_b, receiver);

        assert_eq!(received.len(), 5);
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
