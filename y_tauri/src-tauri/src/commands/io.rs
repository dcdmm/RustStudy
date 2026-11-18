use std::time::{Duration, Instant};

use io_tasks::{UrlCheck, UrlChecker};
use serde::{Deserialize, Serialize};
use tauri::State;
use tauri::ipc::Channel;

use crate::error::{AppResult, ensure_range};

const MAX_URLS: usize = 50;
const MAX_CONCURRENCY: usize = 32;
const MAX_TIMEOUT_MS: u64 = 30_000;

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
