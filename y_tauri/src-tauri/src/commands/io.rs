// 9. IO密集: 调用crates/io_tasks中的tokio并发请求
//
// io_tasks中都是async函数,在async command中直接await即可
// 不需要像CPU密集任务那样交给其他线程池: 等待网络响应时tokio线程会去执行其他任务,不会被占住

use std::time::{Duration, Instant};

use io_tasks::{UrlCheck, UrlChecker};
use serde::{Deserialize, Serialize};
use tauri::State;
use tauri::ipc::Channel;

use crate::error::{AppResult, ensure_range};

// 参数上限
const MAX_URLS: usize = 50;
const MAX_CONCURRENCY: usize = 32;
const MAX_TIMEOUT_MS: u64 = 30_000;

/// 执行方式,前端传字符串"sequential"或"concurrent"
#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum CheckMode {
    Sequential,
    Concurrent,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CheckSummary {
    count: usize,
    total_ms: u64,
}

/// 检测一组网址,每完成一个就通过Channel推送给前端,全部完成后返回总耗时
///
/// 1. UrlChecker在lib.rs的setup中创建并交给Tauri托管,所有请求共用一个HTTP连接池
/// 2. async command的参数中有引用(State<'_, T>)时,返回值必须是Result
#[tauri::command]
pub async fn check_urls(
    checker: State<'_, UrlChecker>,
    urls: Vec<String>,
    mode: CheckMode,
    max_concurrency: usize,
    timeout_ms: u64,
    on_result: Channel<UrlCheck>,
) -> AppResult<CheckSummary> {
    ensure_range("网址数量", urls.len(), 1, MAX_URLS)?;
    ensure_range("maxConcurrency", max_concurrency, 1, MAX_CONCURRENCY)?;
    ensure_range("timeoutMs", timeout_ms, 100, MAX_TIMEOUT_MS)?;

    let timeout = Duration::from_millis(timeout_ms);
    // 每完成一个检测就推送给前端;前端已关闭页面时send会失败,忽略即可
    let send = |check: &UrlCheck| {
        let _ = on_result.send(check.clone());
    };

    let start = Instant::now();
    let results = match mode {
        CheckMode::Sequential => checker.check_sequential(&urls, timeout, send).await,
        CheckMode::Concurrent => {
            checker
                .check_concurrent(&urls, max_concurrency, timeout, send)
                .await
        }
    };
    Ok(CheckSummary {
        count: results.len(),
        total_ms: start.elapsed().as_millis() as u64,
    })
}
