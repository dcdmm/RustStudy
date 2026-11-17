//! 网址检测: 并发请求一组网址,记录每个请求的结果和耗时

use std::error::Error;
use std::sync::Arc;
use std::time::{Duration, Instant};

use serde::Serialize;
use tokio::sync::Semaphore;
use tokio::task::JoinSet;

/// 一个网址的检测结果
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UrlCheck {
    pub url: String,
    pub elapsed_ms: u64,
    pub outcome: Outcome,
}

/// 检测结果的三种情况
///
/// #[serde(tag = "kind")]序列化为{"kind":"response","status":200,"bytes":1234}这样的格式,在TS中对应联合类型
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(tag = "kind", rename_all = "camelCase")]
pub enum Outcome {
    /// 收到了响应(状态码不一定是2xx)
    Response { status: u16, bytes: u64 },
    /// 超时
    Timeout,
    /// 请求失败: 网址格式错误、DNS解析失败、连接被拒绝等
    Failed { message: String },
}

/// 网址检测器
///
/// reqwest::Client内部有连接池,应该创建一次后重复使用(在Tauri中放到托管状态State中)
/// Client内部是Arc,clone很便宜,clone出来的Client共享同一个连接池
#[derive(Clone)]
pub struct UrlChecker {
    client: reqwest::Client,
}

impl UrlChecker {
    pub fn new() -> Result<Self, reqwest::Error> {
        let client = reqwest::Client::builder()
            // 有些网站会拒绝没有User-Agent的请求
            .user_agent(concat!(
                env!("CARGO_PKG_NAME"),
                "/",
                env!("CARGO_PKG_VERSION")
            ))
            .build()?;
        Ok(Self { client })
    }

    /// 检测一个网址: 发送GET请求并读取完整的响应体
    pub async fn check(&self, url: &str, timeout: Duration) -> UrlCheck {
        let start = Instant::now();
        // timeout: 整个请求(建立连接 + 等待响应 + 读取响应体)超过指定时间就放弃
        // 超时后fetch返回的future被drop,请求随之取消,不会在后台继续执行
        let outcome = match tokio::time::timeout(timeout, self.fetch(url)).await {
            Ok(Ok((status, bytes))) => Outcome::Response { status, bytes },
            Ok(Err(err)) => Outcome::Failed {
                message: error_chain(&err),
            },
            Err(_elapsed) => Outcome::Timeout,
        };
        UrlCheck {
            url: url.to_string(),
            elapsed_ms: start.elapsed().as_millis() as u64,
            outcome,
        }
    }

    async fn fetch(&self, url: &str) -> Result<(u16, u64), reqwest::Error> {
        // 每个.await都是一个等待点: 等待期间当前线程会去执行其他任务,而不是傻等
        let response = self.client.get(url).send().await?;
        let status = response.status().as_u16();
        let body = response.bytes().await?;
        Ok((status, body.len() as u64))
    }

    /// 顺序检测: 一个完成后才开始下一个,总耗时约等于所有请求的耗时之和
    pub async fn check_sequential(
        &self,
        urls: &[String],
        timeout: Duration,
        mut on_result: impl FnMut(&UrlCheck),
    ) -> Vec<UrlCheck> {
        let mut results = Vec::with_capacity(urls.len());
        for url in urls {
            let check = self.check(url, timeout).await;
            on_result(&check);
            results.push(check);
        }
        results
    }

    /// 并发检测: 同时发出请求,总耗时约等于最慢的那个请求
    ///
    /// 1. JoinSet: 管理一组并发执行的任务,join_next按完成的先后顺序返回结果
    /// 2. Semaphore: 最多同时执行max_concurrency个请求,避免一次发出太多请求
    pub async fn check_concurrent(
        &self,
        urls: &[String],
        max_concurrency: usize,
        timeout: Duration,
        mut on_result: impl FnMut(&UrlCheck),
    ) -> Vec<UrlCheck> {
        // Semaphore中有max_concurrency个许可,拿到许可的任务才能发请求
        let semaphore = Arc::new(Semaphore::new(max_concurrency.max(1)));
        let mut tasks = JoinSet::new();

        for url in urls {
            // spawn的任务可能比当前函数活得更久,所以不能借用局部变量,要把需要的数据move进去
            let checker = self.clone();
            let semaphore = Arc::clone(&semaphore);
            let url = url.clone();
            tasks.spawn(async move {
                // 没有空闲许可时在这里等待;_permit被drop时(任务结束)自动归还许可
                let _permit = semaphore
                    .acquire_owned()
                    .await
                    .expect("semaphore不会被关闭");
                checker.check(&url, timeout).await
            });
        }

        let mut results = Vec::with_capacity(urls.len());
        // 哪个任务先完成,join_next就先返回哪个
        while let Some(joined) = tasks.join_next().await {
            // 任务panic时返回Err,这里直接跳过
            if let Ok(check) = joined {
                on_result(&check);
                results.push(check);
            }
        }
        results
    }
}

// reqwest的错误信息只有最外层的描述,把source链上的原因都拼起来,才能看出真正的原因
fn error_chain(err: &dyn Error) -> String {
    let mut message = err.to_string();
    let mut source = err.source();
    while let Some(cause) = source {
        message.push_str(": ");
        message.push_str(&cause.to_string());
        source = cause.source();
    }
    message
}

#[cfg(test)]
mod tests {
    use tokio::io::{AsyncReadExt, AsyncWriteExt};
    use tokio::net::TcpListener;

    use super::*;

    const TIMEOUT: Duration = Duration::from_secs(3);

    // 启动一个本地HTTP测试服务器,返回它的地址,测试不依赖外网
    // 请求路径中的数字是返回响应前等待的毫秒数,如GET /200表示等待200ms后返回
    async fn start_server() -> String {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        tokio::spawn(async move {
            loop {
                let (mut socket, _) = listener.accept().await.unwrap();
                // 每个连接一个任务,互不阻塞
                tokio::spawn(async move {
                    let mut buf = [0; 1024];
                    let n = socket.read(&mut buf).await.unwrap_or(0);
                    // 请求的第一行: GET /200 HTTP/1.1
                    let request = String::from_utf8_lossy(&buf[..n]);
                    let delay: u64 = request
                        .split_whitespace()
                        .nth(1)
                        .and_then(|path| path.trim_start_matches('/').parse().ok())
                        .unwrap_or(0);
                    tokio::time::sleep(Duration::from_millis(delay)).await;
                    let response =
                        "HTTP/1.1 200 OK\r\ncontent-length: 2\r\nconnection: close\r\n\r\nok";
                    let _ = socket.write_all(response.as_bytes()).await;
                });
            }
        });
        format!("http://{addr}")
    }

    fn urls(base: &str, delays: &[u64]) -> Vec<String> {
        delays
            .iter()
            .map(|delay| format!("{base}/{delay}"))
            .collect()
    }

    #[tokio::test]
    async fn test01_response() {
        let base = start_server().await;
        let checker = UrlChecker::new().unwrap();
        let check = checker.check(&format!("{base}/0"), TIMEOUT).await;
        println!("{:?}", check); // print->UrlCheck { url: "http://127.0.0.1:xxx/0", elapsed_ms: 3, outcome: Response { status: 200, bytes: 2 } }
        assert_eq!(
            check.outcome,
            Outcome::Response {
                status: 200,
                bytes: 2
            }
        );
    }

    #[tokio::test]
    async fn test02_timeout() {
        let base = start_server().await;
        let checker = UrlChecker::new().unwrap();
        // 服务器500ms后才响应,但只等100ms
        let check = checker
            .check(&format!("{base}/500"), Duration::from_millis(100))
            .await;
        assert_eq!(check.outcome, Outcome::Timeout);
        assert!(check.elapsed_ms < 400);
    }

    #[tokio::test]
    async fn test03_failed() {
        let checker = UrlChecker::new().unwrap();
        let check = checker.check("not-a-url", TIMEOUT).await;
        println!("{:?}", check.outcome); // print->Failed { message: "builder error: relative URL without a base" }
        assert!(matches!(check.outcome, Outcome::Failed { .. }));
    }

    // #[tokio::test]默认使用单线程runtime,并发依然有效: IO并发靠的是等待时切换任务,不需要多个线程
    #[tokio::test]
    async fn test04_sequential_vs_concurrent() {
        let base = start_server().await;
        let checker = UrlChecker::new().unwrap();
        let list = urls(&base, &[200, 200, 200, 200]);

        let start = Instant::now();
        let results = checker.check_sequential(&list, TIMEOUT, |_| {}).await;
        let sequential = start.elapsed();
        assert_eq!(results.len(), 4);

        let start = Instant::now();
        let results = checker.check_concurrent(&list, 4, TIMEOUT, |_| {}).await;
        let concurrent = start.elapsed();
        assert_eq!(results.len(), 4);

        println!("顺序: {sequential:?}, 并发: {concurrent:?}"); // print->顺序: 820ms, 并发: 205ms(大约)
        assert!(sequential >= Duration::from_millis(800));
        assert!(concurrent < Duration::from_millis(500));
    }

    #[tokio::test]
    async fn test05_semaphore_limit() {
        let base = start_server().await;
        let checker = UrlChecker::new().unwrap();
        let list = urls(&base, &[200, 200, 200, 200]);

        // 最多同时2个请求: 4个请求分两批执行,约400ms
        let start = Instant::now();
        checker.check_concurrent(&list, 2, TIMEOUT, |_| {}).await;
        let elapsed = start.elapsed();
        println!("{elapsed:?}"); // print->405ms(大约)
        assert!(elapsed >= Duration::from_millis(400));
        assert!(elapsed < Duration::from_millis(700));
    }

    #[tokio::test]
    async fn test06_completion_order() {
        let base = start_server().await;
        let checker = UrlChecker::new().unwrap();
        let list = urls(&base, &[300, 100, 200]);

        // join_next按完成顺序返回: 等待时间短的先返回
        let mut order = Vec::new();
        checker
            .check_concurrent(&list, 3, TIMEOUT, |check| order.push(check.url.clone()))
            .await;
        assert_eq!(order, urls(&base, &[100, 200, 300]));
    }
}
