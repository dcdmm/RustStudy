fn main() {}

#[cfg(test)]
mod tests {
    use rayon::prelude::*;
    use std::ops::Range;
    use std::time::Instant;
    use tokio::sync::oneshot;
    use tokio::time::{Duration, sleep};

    // cpu密集型任务(是否为素数)
    fn is_prime(n: u64) -> bool {
        if n < 2 {
            return false;
        }
        let mut i = 2;
        while i * i <= n {
            if n % i == 0 {
                return false;
            }
            i += 1;
        }
        true
    }

    // I/O密集型任务
    async fn download(id: u64) -> Range<u64> {
        sleep(Duration::from_millis(id * 200)).await;  // 模拟网络等待
        (id - 1) * 5_000_000..id * 5_000_000
    }

    async fn count_primes(range: Range<u64>) -> usize {
        // 直接调用par_iter时,当前线程要等到计算结束才能继续,期间无法执行其他异步任务
        // rayon::spawn把计算交给rayon线程池后立即返回,当前线程不用等待
        let (sender, receiver) = oneshot::channel();
        rayon::spawn(move || {
            let count = range.into_par_iter().filter(|n| is_prime(*n)).count();
            let _ = sender.send(count);
        });
        // 等待结果时让出线程,其他异步任务可以继续执行
        receiver.await.unwrap()
    }

    async fn handle_request(id: u64, start: Instant) -> usize {
        let range = download(id).await;
        println!("请求{id}下载完成: {}毫秒", start.elapsed().as_millis());
        let count = count_primes(range).await;
        println!(
            "请求{id}计算完成: {}毫秒,素数个数: {count}",
            start.elapsed().as_millis()
        );
        count
    }

    #[tokio::test]
    async fn test_io_and_cpu() {
        let start = Instant::now();
        let handles: Vec<_> = (1..=3)
            .map(|id| tokio::spawn(handle_request(id, start)))
            .collect();

        let mut total = 0;
        for handle in handles {
            total += handle.await.unwrap();
        }
        println!(
            "总耗时: {}毫秒,素数总数: {total}",
            start.elapsed().as_millis()
        );
        assert_eq!(total, 970_704);
    }
}
