use serde::Serialize;
use std::sync::Mutex;
use std::time::Instant;
use tauri::{AppHandle, Emitter, Manager, State};

#[derive(Clone, Serialize)]
pub struct PrimeResult {
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

// 启动后台计算后立即返回,完成后通过job-changed事件通知前端
#[tauri::command]
pub fn start_job(app: AppHandle, state: State<JobState>, limit: u64) {
    let mut job = state.job.lock().unwrap();
    job.running = true;
    // payload为最新状态,前端据此更新页面
    app.emit("job-changed", job.clone()).unwrap();

    // 不await,command立即返回,计算在后台继续
    tauri::async_runtime::spawn_blocking(move || {
        let start = Instant::now();
        // 使用单线程计算,耗时较长,便于切换页面观察
        let count = heavy::count_primes_seq(limit);
        let ms = start.elapsed().as_millis() as u64;
        // command参数中的State无法移入线程,需通过AppHandle重新获取
        let state = app.state::<JobState>();
        let mut job = state.job.lock().unwrap();
        job.results.push(PrimeResult { count, ms });
        job.running = false;
        app.emit("job-changed", job.clone()).unwrap();
    });
}
