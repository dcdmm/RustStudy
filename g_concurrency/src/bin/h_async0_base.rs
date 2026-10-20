fn main() {}

#[cfg(test)]
mod tests {
    #[tokio::test]
    async fn test_01_base() {
        // 异步函数
        async fn answer() -> u32 {
            println!("异步函数开始执行");
            42
        }

        let future = answer();

        let value = future.await;
        println!("异步函数结果:{value}");
        assert_eq!(value, 42);

        let base = 40;
        // 异步代码块
        let future_block = async { base + 2 };
        let value_block = future_block.await;
        println!("异步代码块结果:{value_block}");
        assert_eq!(value_block, 42);
    }

    #[tokio::test]
    async fn test_03_await_chain() {
        struct Number(u32);

        impl Number {
            async fn double(self) -> u32 {
                self.0 * 2
            }
        }

        async fn get_number() -> Number {
            Number(21)
        }

        // 调式调用
        let chained_result = get_number().await.double().await;
        println!("链式调用结果:{chained_result}");
    }
}
