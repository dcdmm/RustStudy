use serde::Serialize;
use std::time::Duration;
use tauri::{AppHandle, Emitter};

// 事件数据,emit要求实现Serialize和Clone
#[derive(Clone, Serialize)]
pub struct Tick {
    seconds: u32,
    message: String,
}

// 每秒向前端发送一次tick事件
pub fn start_ticker(app: AppHandle) {
    // 在新线程中循环,避免阻塞启动
    std::thread::spawn(move || {
        let mut seconds = 0;
        loop {
            std::thread::sleep(Duration::from_secs(1));
            seconds += 1;
            let tick = Tick {
                seconds,
                message: format!("第{}次发送", seconds),
            };
            // 发送tick事件,前端每收到一次就执行一次回调
            // 页面未打开时无人接收,该次通知丢失;线程不受影响,继续计数
            app.emit("tick", tick).unwrap();
        }
    });
}
