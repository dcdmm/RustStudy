use serde::Serialize;
use std::time::Instant;

// 返回给前端的计算结果
#[derive(Serialize)]
pub struct PrimeResult {
    count: u64,
    ms: u64,
}

#[tauri::command]
pub async fn count_primes(limit: u64, parallel: bool) -> Result<PrimeResult, String> {
    // 耗时计算的三种写法:
    // | 写法                      | 占用线程     | 窗口 | 其他异步任务 |
    // |---------------------------|--------------|------|--------------|
    // | 1. 同步command            | 主线程       | 卡住 | -            |
    // | 2. async直接计算          | 异步任务线程 | 不卡 | 需等待       |
    // | 3. async + spawn_blocking | 专用线程     | 不卡 | 不受影响     |
    tauri::async_runtime::spawn_blocking(move || {
        let start = Instant::now();
        let count = if parallel {
            heavy::count_primes_par(limit)
        } else {
            heavy::count_primes_seq(limit)
        };
        PrimeResult {
            count,
            ms: start.elapsed().as_millis() as u64,
        }
    })
    .await
    .map_err(|e| e.to_string())
}
