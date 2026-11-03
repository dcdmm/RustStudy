fn main() {}

#[cfg(test)]
mod tests {
    use std::time::Instant;
    use tokio::sync::mpsc::{self, error::TryRecvError};
    use tokio::time::{Duration, sleep};

    #[tokio::test]
    async fn test01_base() {
        // Creates an unbounded mpsc channel for communicating between asynchronous tasks without backpressure.
        let (tx, mut rx) = mpsc::unbounded_channel(); // 对应标准库: std::sync::mpsc::channel()
        let message = String::from("hello async");

        tx.send(message).unwrap();
        drop(tx);

        let received = rx.recv().await; // recv是异步函数
        println!("收到:{received:?}");

        // Receives the next value for this receiver.
        // This method returns `None` if the channel has been closed and there are no remaining messages in the channel's buffer. 
        assert_eq!(rx.recv().await, None);
    }

    #[tokio::test]
    async fn test02() {
        let (tx, mut rx) = mpsc::unbounded_channel(); 
        let start = Instant::now();

        // 依次发送"hi"、"from"、"async"
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
    }

    #[tokio::test]
    async fn test03() {
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
                let value = rx.recv().await.unwrap(); // 通道为空时挂起,join!切换到sender执行
                println!("接收:{value}");
                received.push(value);
            }
            received
        };

        // 边发边收
        let ((), received) = tokio::join!(sender, receiver);
        assert_eq!(received, [10, 20, 30]);

        assert_eq!(rx.try_recv(), Err(TryRecvError::Empty));
        println!("发送Future已完成,但外层tx还存在,通道仍未关闭");
        drop(tx);
        assert_eq!(rx.recv().await, None);
    }

    #[tokio::test]
    async fn test03_move() {
        let (tx, mut rx) = mpsc::unbounded_channel();
        let start = Instant::now();

        let sender = async move { // tx的所有权移入sender
            for value in ["hi", "from", "async"] {
                tx.send(value).unwrap();
                sleep(Duration::from_millis(20)).await;
            }
        }; // sender执行完毕时tx随之drop,通道关闭
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
    async fn test04_multiple_producers() {
        let (tx, mut rx) = mpsc::unbounded_channel();
        let tx1 = tx.clone(); // clone得到多个发送端(共用同一个通道)

        let sender_a = async move {
            for value in ["A1", "A2", "A3"] {
                tx1.send(value).unwrap();
                sleep(Duration::from_millis(20)).await;
            }
        }; // 释放tx1

        let receiver = async {
            let mut received = Vec::new();
            while let Some(value) = rx.recv().await { // tx1和tx都释放后才返回None
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
        }; // 释放tx

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
