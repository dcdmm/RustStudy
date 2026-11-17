//! IO密集型任务: 用tokio异步并发执行网络请求
//!
//! IO密集型任务的大部分时间花在等待上(等待建立连接、等待服务器响应)
//! tokio在一个任务等待时去执行其他任务,少量线程就能同时处理大量请求
//!
//! 演示的tokio用法(都在url_check.rs中):
//! 1. async/await: 异步函数
//! 2. tokio::time::timeout: 给异步操作设置超时
//! 3. JoinSet: 并发执行多个任务,哪个先完成就先处理哪个
//! 4. Semaphore: 限制同时执行的任务数量
//!
//! 本crate不依赖Tauri,可以单独测试和使用

mod url_check;

pub use url_check::{Outcome, UrlCheck, UrlChecker};
