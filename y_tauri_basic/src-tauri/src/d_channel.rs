use std::time::Duration;
use tauri::ipc::Channel;

// Channel由前端创建并传入,用于向前端多次发送消息
#[tauri::command]
pub async fn run_task(on_progress: Channel<u32>) -> String {
    for i in 1..=10 {
        tokio::time::sleep(Duration::from_millis(300)).await;
        // 发送当前进度
        on_progress.send(i * 10).unwrap();
    }
    "任务完成".to_string()
}
