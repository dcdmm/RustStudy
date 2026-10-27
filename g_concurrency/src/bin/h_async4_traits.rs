fn main() {}

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::future::{Future, poll_fn};
    use std::pin::{Pin, pin};
    use std::task::{Context, Poll, Waker};
    use tokio::sync::mpsc;
    use tokio::time::{Duration, Instant, sleep};
    use tokio_stream::{Stream, StreamExt};

    // 手写 Future:需要被 poll 若干次才完成,并记录被 poll 的次数
    struct CountDown {
        remaining: u32,
        polls: u32,
    }

    impl CountDown {
        fn new(remaining: u32) -> Self {
            Self {
                remaining,
                polls: 0,
            }
        }
    }

    impl Future for CountDown {
        type Output = &'static str;

        fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
            // CountDown 自动实现 Unpin,所以 Pin<&mut Self> 可以像 &mut Self 一样修改字段
            self.polls += 1;
            if self.remaining == 0 {
                return Poll::Ready("倒计时完成");
            }
            self.remaining -= 1;
            // 返回 Pending 前通过 Context 中的 waker 通知 runtime 稍后再 poll
            // 如果不调用 wake,runtime 不知道何时再检查它,.await 会一直挂起
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }

    // 手写 Stream:每产生一项之前先返回一次 Pending,模拟数据随时间到达
    struct Counter {
        current: u32,
        max: u32,
        waiting: bool,
    }

    impl Counter {
        fn new(max: u32) -> Self {
            Self {
                current: 0,
                max,
                waiting: false,
            }
        }
    }

    impl Stream for Counter {
        type Item = u32;

        fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<u32>> {
            if self.current == self.max {
                // 内层 None 表示流已经结束
                return Poll::Ready(None);
            }
            if !self.waiting {
                // 外层 Pending 表示下一项还没就绪
                self.waiting = true;
                cx.waker().wake_by_ref();
                return Poll::Pending;
            }
            self.waiting = false;
            self.current += 1;
            Poll::Ready(Some(self.current))
        }
    }

    // 简化版 join_all:并发 poll 集合中的全部 Future,全部完成后按原顺序返回结果
    // poll 要求 Pin<&mut Self>,这里限制 F: Unpin,才能用 Pin::new 安全地得到它
    async fn join_all<F: Future + Unpin>(futures: Vec<F>) -> Vec<F::Output> {
        // None 表示该 Future 已经完成,完成后不能再 poll
        let mut futures: Vec<Option<F>> = futures.into_iter().map(Some).collect();
        let mut outputs: Vec<Option<F::Output>> = futures.iter().map(|_| None).collect();

        // poll_fn 用闭包创建 Future,闭包就是它的 poll 方法
        poll_fn(|cx| {
            let mut all_ready = true;
            for (slot, output) in futures.iter_mut().zip(outputs.iter_mut()) {
                if let Some(future) = slot {
                    match Pin::new(future).poll(cx) {
                        Poll::Ready(value) => {
                            *output = Some(value);
                            *slot = None;
                        }
                        Poll::Pending => all_ready = false,
                    }
                }
            }
            if all_ready {
                Poll::Ready(outputs.iter_mut().map(|o| o.take().unwrap()).collect())
            } else {
                Poll::Pending
            }
        })
        .await
    }

    // 编译期检查:只有实现 Unpin 的类型才能调用
    fn assert_unpin<T: Unpin>(_: &T) {}

    #[test]
    fn test_01_manual_poll_pending_then_ready() {
        // 不借助 runtime,手动 poll,观察 Poll 的两种结果
        // Waker::noop 什么也不做,只用于构造 Context
        let mut cx = Context::from_waker(Waker::noop());
        let mut countdown = CountDown::new(2);

        // CountDown 是 Unpin,可以用 Pin::new 包裹 &mut 引用
        let first = Pin::new(&mut countdown).poll(&mut cx);
        let second = Pin::new(&mut countdown).poll(&mut cx);
        let third = Pin::new(&mut countdown).poll(&mut cx);
        println!("三次poll:{first:?},{second:?},{third:?}");

        assert_eq!(first, Poll::Pending);
        assert_eq!(second, Poll::Pending);
        assert_eq!(third, Poll::Ready("倒计时完成"));
        assert_eq!(countdown.polls, 3);
    }

    #[tokio::test]
    async fn test_02_await_lets_runtime_poll_until_ready() {
        let mut countdown = CountDown::new(3);

        // .await 底层就是 poll:返回 Pending 时让出控制权,被唤醒后 runtime 再次 poll
        // 对于 Unpin 的 Future F,&mut F 也实现了 Future,await 后还能查看原值
        let output = (&mut countdown).await;
        println!("结果:{output},runtime共poll了{}次", countdown.polls);

        assert_eq!(output, "倒计时完成");
        assert_eq!(countdown.polls, 4);
    }

    #[test]
    fn test_03_async_block_is_a_state_machine() {
        let mut cx = Context::from_waker(Waker::noop());
        let data = vec![1, 2, 3];

        // 编译器把 async 块编译成状态机,每个 await 点对应一个状态
        // 跨越 await 点的变量和引用都要保存在状态机里,这里 first 引用了同样保存其中的 numbers
        // 这种指向自身字段的引用就是 self-referential,移动状态机会让引用指向旧地址
        let mut future = pin!(async move {
            let numbers = data;
            let first = &numbers[0];
            CountDown::new(1).await;
            *first * 100
        });
        // async 块是 !Unpin 的,不能用 Pin::new(&mut future)
        // pin! 把它固定在当前栈帧上,得到 Pin<&mut 匿名类型>

        // 外层 Future 被 poll 时,会转去 poll 内部正在 await 的 CountDown
        let first = future.as_mut().poll(&mut cx);
        let second = future.as_mut().poll(&mut cx);
        println!("async块的两次poll:{first:?},{second:?}");

        assert_eq!(first, Poll::Pending);
        assert_eq!(second, Poll::Ready(100));
    }

    #[test]
    #[should_panic(expected = "resumed after completion")]
    fn test_04_poll_after_ready_panics() {
        let mut cx = Context::from_waker(Waker::noop());
        let mut future = pin!(async { 42 });

        assert_eq!(future.as_mut().poll(&mut cx), Poll::Ready(42));
        // 返回 Ready 之后不应再 poll,async 块再次被 poll 会 panic
        // 这和 Iterator 返回 None 之后的约定类似,所以 join_all 要记录哪些 Future 已完成
        let _ = future.as_mut().poll(&mut cx);
    }

    #[tokio::test]
    async fn test_05_box_dyn_future_needs_pin() {
        let events = RefCell::new(Vec::new());
        let a = async {
            sleep(Duration::from_millis(10)).await;
            events.borrow_mut().push("A");
        };
        let b = async {
            events.borrow_mut().push("B");
        };

        // 每个 async 块都有独一无二的匿名类型,即使 Output 都是 (),也不能直接放进同一个 Vec
        // let futures = vec![a, b]; // 编译错误:mismatched types

        // 用 Box 转成 trait 对象仍然不够:
        // let futures: Vec<Box<dyn Future<Output = ()>>> = vec![Box::new(a), Box::new(b)];
        // join_all(futures).await;
        // 编译错误:`dyn Future<Output = ()>` cannot be unpinned
        // 原因是 Box<T> 只有在 T: Future + Unpin 时才实现 Future

        // Box::pin 把值放到堆上并固定,Pin<Box<T>> 本身是 Unpin 的,也实现了 Future
        let futures: Vec<Pin<Box<dyn Future<Output = ()> + '_>>> = vec![Box::pin(a), Box::pin(b)];
        join_all(futures).await;

        let events = events.into_inner();
        println!("执行顺序:{events:?}");
        // 两个 Future 并发执行,A 等待期间 B 先完成
        assert_eq!(events, ["B", "A"]);
    }

    #[tokio::test]
    async fn test_06_pin_macro_with_join_all() {
        // 对应书中 Listing 17-24:用 pin! 固定在栈上,不需要堆分配
        let (tx, mut rx) = mpsc::unbounded_channel();
        let received = RefCell::new(Vec::new());

        let tx1 = tx.clone();
        let tx1_fut = pin!(async move {
            for val in ["hi", "from", "the", "future"] {
                tx1.send(val).unwrap();
                sleep(Duration::from_millis(5)).await;
            }
        });

        let rx_fut = pin!(async {
            // 两个发送端都释放后 recv 返回 None,循环结束
            while let Some(value) = rx.recv().await {
                received.borrow_mut().push(value);
            }
        });

        let tx_fut = pin!(async move {
            for val in ["more", "messages", "for", "you"] {
                tx.send(val).unwrap();
                sleep(Duration::from_millis(5)).await;
            }
        });

        // Pin<&mut T> 是 Unpin 的,而且在 T: Future 时实现 Future
        // pin! 只能固定在当前作用域内,不能把这个 Vec 返回到函数外
        let futures: Vec<Pin<&mut dyn Future<Output = ()>>> = vec![tx1_fut, rx_fut, tx_fut];
        join_all(futures).await;

        // pin! 固定的 async 块要到作用域结束才释放,它仍借用着 received
        // 所以这里不能像前面那样调用 into_inner 取走所有权,只能 borrow
        let received = received.borrow().clone();
        println!("接收顺序:{received:?}");
        // 两个发送端交替发送,只断言数量和内容,不依赖计时器的先后
        assert_eq!(received.len(), 8);
        assert!(received.contains(&"hi") && received.contains(&"you"));
    }

    // 不同分支返回不同的 async 块,用 trait 对象统一返回类型
    // 需要离开当前作用域时用 Box::pin,如果要交给 tokio::spawn 还需要加 + Send
    fn greeting(lang: &str) -> Pin<Box<dyn Future<Output = String>>> {
        match lang {
            "zh" => Box::pin(async {
                sleep(Duration::from_millis(5)).await;
                String::from("你好")
            }),
            _ => Box::pin(async { String::from("hello") }),
        }
    }

    #[tokio::test]
    async fn test_07_box_pin_can_leave_scope() {
        let futures = vec![greeting("zh"), greeting("en")];
        let outputs = join_all(futures).await;
        println!("问候语:{outputs:?}");
        assert_eq!(outputs, ["你好", "hello"]);
    }

    #[test]
    fn test_08_unpin_values_can_move_inside_pin() {
        // Unpin 是 marker trait,和 Send,Sync 一样没有方法,编译器会自动实现
        // 大多数类型都是 Unpin,只有像 async 块状态机这样的类型才是 !Unpin
        let mut text = String::from("hello");
        assert_unpin(&text);
        assert_unpin(&CountDown::new(0));
        // Box<T> 和 Pin<&mut T> 无论 T 是什么都是 Unpin,被固定的是它们指向的值
        assert_unpin(&Box::pin(async {}));
        // assert_unpin(&async {}); // 编译错误:async 块没有实现 Unpin

        // String 没有内部引用,被 Pin 包裹后仍然可以整体替换,不违反 Pin 的约定
        let mut pinned = Pin::new(&mut text);
        pinned.set(String::from("world"));
        pinned.push('!');
        // 对 Unpin 类型,还可以取回原来的 &mut 引用
        let inner: &mut String = Pin::into_inner(pinned);
        inner.push('?');

        println!("替换后的值:{text}");
        assert_eq!(text, "world!?");
    }

    #[tokio::test]
    async fn test_09_tokio_sleep_is_not_unpin() {
        // 补充 Tokio 中常见的 !Unpin 类型:Sleep 内部有计时器节点,不能被移动
        let sleep_fut = sleep(Duration::from_millis(5));
        // assert_unpin(&sleep_fut); // 编译错误:PhantomPinned 没有实现 Unpin
        // (&mut sleep_fut).await; // 编译错误:&mut F 只有在 F: Unpin 时才实现 Future

        // 想在 await 之后继续使用同一个 Sleep,先用 pin! 固定,再通过 as_mut 重复借用
        let mut sleep_fut = pin!(sleep_fut);
        sleep_fut.as_mut().await;
        assert!(sleep_fut.is_elapsed());

        // reset 的 receiver 是 self: Pin<&mut Self>,只能在固定后调用
        let start = Instant::now();
        sleep_fut.as_mut().reset(start + Duration::from_millis(5));
        assert!(!sleep_fut.is_elapsed());
        sleep_fut.as_mut().await;
        println!("重置后又等待了:{:?}", start.elapsed());
        assert!(sleep_fut.is_elapsed());
    }

    #[test]
    fn test_10_stream_poll_next_has_two_layers() {
        // Stream 结合了 Iterator 和 Future:poll_next 返回 Poll<Option<Item>>
        // 标准库还没有 Stream,tokio_stream::Stream 是 futures-core 中同一个 trait 的重新导出
        let mut cx = Context::from_waker(Waker::noop());
        let mut counter = Counter::new(2);
        let mut results = Vec::new();
        loop {
            let result = Pin::new(&mut counter).poll_next(&mut cx);
            results.push(result);
            if result == Poll::Ready(None) {
                break;
            }
        }
        println!("每次poll_next:{results:?}");

        assert_eq!(
            results,
            [
                Poll::Pending,
                Poll::Ready(Some(1)),
                Poll::Pending,
                Poll::Ready(Some(2)),
                Poll::Ready(None),
            ]
        );
    }

    #[tokio::test]
    async fn test_11_stream_ext_methods_for_free() {
        // StreamExt 对所有实现 Stream 的类型自动实现
        // 自定义流只需实现 poll_next,就能使用 next,map,filter,collect 等便捷方法
        let mut counter = Counter::new(3);
        assert_eq!(counter.next().await, Some(1));

        let rest: Vec<_> = counter.map(|n| n * 10).collect().await;
        println!("剩余项乘以10:{rest:?}");
        assert_eq!(rest, [20, 30]);
    }

    #[tokio::test]
    async fn test_12_stream_next_requires_unpin() {
        // StreamExt::next 的签名:fn next(&mut self) -> Next<'_, Self> where Self: Unpin
        // timeout 返回的流内部包含 Sleep,所以它是 !Unpin 的
        let stream = tokio_stream::iter([1, 2]).timeout(Duration::from_secs(1));
        // stream.next().await; // 编译错误:`PhantomPinned` cannot be unpinned

        // 修复方法和 Future 相同:pin! 固定在栈上,或者 Box::pin 放到堆上
        // Pin<&mut S> 是 Unpin 的,并且在 S: Stream 时也实现 Stream
        let mut stream = pin!(stream);
        let first = stream.next().await.unwrap().unwrap();
        let second = stream.next().await.unwrap().unwrap();
        println!("固定后读取:{first},{second}");

        assert_eq!((first, second), (1, 2));
        assert!(stream.next().await.is_none());
    }
}
