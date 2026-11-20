use serde::Serialize;
use std::sync::Mutex;
use std::time::Instant;
use tauri::{AppHandle, Emitter, State};

// 返回给前端的计算结果
#[derive(Clone, Serialize)]
pub struct PrimeResult {
    parallel: bool,
    count: u64,
    ms: u64,
}

// 任务状态,前端通过get_job和job-changed事件获取
#[derive(Default, Clone, Serialize)]
pub struct Job {
    running: bool,
    results: Vec<PrimeResult>,
}

// 由Tauri托管,前端页面卸载后仍然保留
#[derive(Default)]
pub struct JobState {
    job: Mutex<Job>,
}

// 前端打开页面时调用,获取当前任务状态
#[tauri::command]
pub fn get_job(state: State<JobState>) -> Job {
    state.job.lock().unwrap().clone()
}

// 结果不通过返回值传递,而是写入JobState并发送job-changed事件,payload为最新状态
#[tauri::command]
pub async fn count_primes(
    app: AppHandle,
    state: State<'_, JobState>,
    limit: u64,
    parallel: bool,
) -> Result<(), String> {
    // 锁须在await前释放,因此放在单独的代码块中
    {
        let mut job = state.job.lock().unwrap();
        job.running = true;
        app.emit("job-changed", job.clone()).unwrap();
    }

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
    app.emit("job-changed", job.clone()).unwrap();
    Ok(())
}
