fn main() {}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use tokio::sync::mpsc;
    use tokio::time::{Duration, sleep, timeout};
    use tokio_stream::{StreamExt, wrappers::UnboundedReceiverStream};

    #[tokio::test]
    async fn test01_base() {
        let mut iter = [10, 20].into_iter();
        assert_eq!(iter.next(), Some(10));
        assert_eq!(iter.next(), Some(20));
        assert_eq!(iter.next(), None);

        let mut stream = tokio_stream::iter([10, 20]);
        // next返回的是Future,这里还没拿到数据
        let next_value = stream.next();
        // await之后才拿到数据
        let first = next_value.await;
        println!("第一次获取:{first:?}");
        assert_eq!(first, Some(10));
        assert_eq!(stream.next().await, Some(20));
        assert_eq!(stream.next().await, None);

        let values = [1, 2, 3, 4, 5];
        let iter = values.iter().map(|n| n * 2);
        let mut stream = tokio_stream::iter(iter);

        let mut received = Vec::new();
        while let Some(value) = stream.next().await {
            println!("收到:{value}");
            received.push(value);
        }
        assert_eq!(received, [2, 4, 6, 8, 10]);
    }

    #[tokio::test]
    async fn test02_stream_no_yield() {
        let events = RefCell::new(Vec::new());
        let reader = async {
            let mut stream = tokio_stream::iter(["A1", "A2", "A3"]);
            // 数据已在数组中,next立即返回,reader不会暂停
            while let Some(value) = stream.next().await {
                events.borrow_mut().push(value);
                // 在push后面加sleep才会真正暂停,顺序变成A1,B,A2,A3
                // sleep(Duration::from_secs(1)).await;
            }
        };
        let other = async {
            // reader运行完才轮到other,所以B排在最后
            events.borrow_mut().push("B");
        };

        tokio::join!(reader, other);
        let events = events.into_inner();
        println!("执行顺序:{events:?}");
        assert_eq!(events, ["A1", "A2", "A3", "B"]);
    }

    #[tokio::test]
    async fn test03_channel_to_stream() {
        let (tx, rx) = mpsc::unbounded_channel();
        // rx不是Stream,包装后才能使用Stream的方法(如next,filter,map)
        let mut stream = UnboundedReceiverStream::new(rx);
        let sender = async move {
            for message in ["你好", "订单已发货", "订单已送达"] {
                sleep(Duration::from_millis(20)).await;
                println!("发送:{message}");
                tx.send(message).unwrap();
            }
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

        let (received, ()) = tokio::join!(biased; receiver, sender);
        assert_eq!(received, ["你好", "订单已发货", "订单已送达"]);

        let (tx, rx) = mpsc::unbounded_channel();
        for value in [1, 2, 3, 4, 5, 6] {
            tx.send(value).unwrap();
        }
        drop(tx);
        // 先筛出2,4,6,再变成20,40,60,最后只取前两项
        let stream = UnboundedReceiverStream::new(rx)
            .filter(|value| *value % 2 == 0)
            .map(|value| value * 10)
            .take(2);

        // collect返回Future,等待Stream结束后得到集合
        // 聊天消息,实时行情,服务器请求这类数据没有终点,Stream不会结束,这时collect永远不会返回,应使用while let逐项处理
        let received: Vec<_> = stream.collect().await;
        println!("筛选和转换后:{received:?}");
        assert_eq!(received, [20, 40]);
    }

    #[tokio::test]
    async fn test04_stream_end() {
        let (tx, rx) = mpsc::unbounded_channel();
        let mut stream = UnboundedReceiverStream::new(rx);

        // 此时没有发送消息,tx也未释放,next不会返回None,只能持续等待
        // 用timeout只等20毫秒,防止测试卡住
        let result = timeout(Duration::from_millis(20), stream.next()).await;
        println!("没有消息,等待超时:{result:?}");
        assert!(result.is_err());

        // timeout只取消了本次next,tx和stream未受影响,channel仍可继续收发
        tx.send(42).unwrap();
        drop(tx);

        // tx已释放,但队列里还有42,next仍能取到
        assert_eq!(stream.next().await, Some(42));
        // 队列已空且tx已释放,next才返回None
        assert_eq!(stream.next().await, None);
    }
}
