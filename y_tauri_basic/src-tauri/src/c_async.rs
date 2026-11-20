use std::time::Duration;

// 普通command在主线程执行,等待期间整个窗口卡住
#[tauri::command]
pub fn slow_sync() -> String {
    std::thread::sleep(Duration::from_secs(3));
    "同步任务完成".to_string()
}

// async command在后台执行,等待期间窗口照常响应
#[tauri::command]
pub async fn slow_async() -> String {
    tokio::time::sleep(Duration::from_secs(3)).await;
    "异步任务完成".to_string()
}
