fn main() {}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use tokio::sync::mpsc;
    use tokio::time::{Duration, sleep, timeout};
    use tokio_stream::{StreamExt, wrappers::UnboundedReceiverStream};

    #[tokio::test]
    async fn test_01_iterator_next_and_stream_next() {
        let mut iter = [10, 20].into_iter();
        assert_eq!(iter.next(), Some(10));
        assert_eq!(iter.next(), Some(20));
        assert_eq!(iter.next(), None);

        let mut stream = tokio_stream::iter([10, 20]);
        let next_value = stream.next();
        let first = next_value.await;
        println!("第一次获取:{first:?}");
        assert_eq!(first, Some(10));
        assert_eq!(stream.next().await, Some(20));
        assert_eq!(stream.next().await, None);
    }

    #[tokio::test]
    async fn test_02_stream_from_iterator() {
        let values = [1, 2, 3, 4, 5];
        let iter = values.iter().map(|n| n * 2);
        let mut stream = tokio_stream::iter(iter);

        let mut received = Vec::new();
        // 每次获取一项,得到 Some 就处理,得到 None 就结束循环
        while let Some(value) = stream.next().await {
            println!("收到:{value}");
            received.push(value);
        }
        assert_eq!(received, [2, 4, 6, 8, 10]);
    }

    #[tokio::test]
    async fn test_03_ready_stream_does_not_yield() {
        // 补充边界:把现成数据转成 Stream,不会自动产生等待或者启动后台任务
        let events = RefCell::new(Vec::new());
        let reader = async {
            let mut stream = tokio_stream::iter(["A1", "A2", "A3"]);
            while let Some(value) = stream.next().await {
                events.borrow_mut().push(value);
            }
        };
        let other = async {
            events.borrow_mut().push("B");
        };

        // 固定先检查 reader,三项数据都已就绪,每次 .await 都直接得到结果
        tokio::join!(biased; reader, other);
        let events = events.into_inner();
        println!("执行顺序:{events:?}");
        assert_eq!(events, ["A1", "A2", "A3", "B"]);
    }

    #[tokio::test]
    async fn test_04_channel_messages_arrive_over_time() {
        let (tx, rx) = mpsc::unbounded_channel();
        // 将以前用 recv() 读取的接收端包装成 Stream,统一用 next() 读取
        let mut stream = UnboundedReceiverStream::new(rx);
        let sender = async move {
            for message in ["你好", "订单已发货", "订单已送达"] {
                sleep(Duration::from_millis(20)).await;
                println!("发送:{message}");
                tx.send(message).unwrap();
            }
            // async move 拥有 tx,发送完成后释放最后一个发送端
        };
        let receiver = async {
            let mut received = Vec::new();
            while let Some(message) = stream.next().await {
                println!("接收:{message}");
                received.push(message);
            }
            println!("消息全部取完且发送端已释放,数据流结束");
            received
        };

        // 先尝试接收,没有消息时暂停 receiver,让 sender 有机会推进
        let (received, ()) = tokio::join!(biased; receiver, sender);
        assert_eq!(received, ["你好", "订单已发货", "订单已送达"]);
    }

    #[tokio::test]
    async fn test_05_empty_channel_is_not_end_of_stream() {
        let (tx, rx) = mpsc::unbounded_channel();
        let mut stream = UnboundedReceiverStream::new(rx);

        // 通道暂时为空,但 tx 还存在,next().await 会继续等,不会返回 None
        // 用 Tokio 的 timeout 限制这次等待,避免演示一直挂起
        let result = timeout(Duration::from_millis(20), stream.next()).await;
        println!("没有消息,等待超时:{result:?}");
        assert!(result.is_err());

        // 这里取消一次等待不会关闭通道,后续仍然可以发送和接收
        tx.send(42).unwrap();
        drop(tx);

        // 即使发送端已经释放,仍要先取完队列中已有的数据
        assert_eq!(stream.next().await, Some(42));
        assert_eq!(stream.next().await, None);
    }

    #[tokio::test]
    async fn test_06_stream_ext_transforms_items() {
        // 补充 StreamExt 的便利方法,这里的 map 和 filter 处理的是 Stream
        let stream = tokio_stream::iter([1, 2, 3, 4, 5, 6])
            .filter(|value| *value % 2 == 0)
            .map(|value| value * 10)
            .take(2);

        // 先筛出 2,4,6,再变成 20,40,60,最后只取前两项
        // collect 返回 Future,等待流结束后得到集合
        let received: Vec<_> = stream.collect().await;
        println!("筛选和转换后:{received:?}");
        assert_eq!(received, [20, 40]);
        // 对于持续产生数据的流,逐项处理可以避免等待全部数据或者无限收集
    }
}
