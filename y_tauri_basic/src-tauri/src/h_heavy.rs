use serde::Serialize;
use std::sync::Mutex;
use std::time::Instant;
use tauri::{AppHandle, Emitter, State};

#[derive(Clone, Serialize)]
pub struct PrimeResult {
    parallel: bool,
    count: u64,
    ms: u64,
}

#[derive(Default, Clone, Serialize)]
pub struct Job {
    running: bool,
    results: Vec<PrimeResult>,
}

#[derive(Default)]
pub struct JobState {
    job: Mutex<Job>,
}

#[tauri::command]
pub fn get_job(state: State<JobState>) -> Job {
    state.job.lock().unwrap().clone()
}

#[tauri::command]
pub async fn count_primes(
    app: AppHandle,
    state: State<'_, JobState>,
    limit: u64,
    parallel: bool,
) -> Result<(), String> {
    // 锁须在await前释放,否则编译不通过
    {
        let mut job = state.job.lock().unwrap();
        job.running = true;
        // 通知前端计算开始,页面显示计算中
        app.emit("job-changed", job.clone()).unwrap();
    } // 锁在此释放

    // 耗时计算的三种写法:
    // | 写法                      | 占用线程     | 窗口 | 其他异步任务 |
    // |---------------------------|--------------|------|--------------|
    // | 1. 同步command            | 主线程       | 卡住 | -            |
    // | 2. async直接计算          | 异步任务线程 | 不卡 | 需等待       |
    // | 3. async + spawn_blocking | 专用线程     | 不卡 | 不受影响     |
    let result = tauri::async_runtime::spawn_blocking(move || {
        let start = Instant::now();
        let count = if parallel {
            heavy::count_primes_par(limit)
        } else {
            heavy::count_primes_seq(limit)
        };
        PrimeResult {
            parallel,
            count,
            ms: start.elapsed().as_millis() as u64,
        }
    })
    .await
    .map_err(|e| e.to_string())?;

    let mut job = state.job.lock().unwrap();
    job.results.push(result);
    job.running = false;
    // 通知前端计算完成;结果已存入JobState,即使页面未打开导致通知丢失,返回页面时也可通过get_job取回
    app.emit("job-changed", job.clone()).unwrap();
    Ok(())
}
