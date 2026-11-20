use serde::Serialize;
use std::time::Instant;

// 返回给前端的计算结果
#[derive(Serialize)]
pub struct PrimeResult {
    count: u64,
    ms: u64,
}

// 前端invoke("count_primes", { limit, parallel })调用
#[tauri::command]
pub async fn count_primes(limit: u64, parallel: bool) -> Result<PrimeResult, String> {
    // async command运行在处理异步任务的线程上,长时间计算会占住该线程
    // spawn_blocking将计算移到专用线程执行,await等待其完成
    tauri::async_runtime::spawn_blocking(move || {
        let start = Instant::now();
        // 计算逻辑位于crates/heavy,此处只负责调用
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
