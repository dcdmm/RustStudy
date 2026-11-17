// 示例5: 事件(Rust主动向前端广播消息)
// 参考文档: https://tauri.app/develop/calling-frontend/#event-system
//
// 事件和Channel的区别:
// 1. Channel: 属于某一次command调用,由前端发起,用来推送这次调用的进度
// 2. 事件: 不依赖command,后端随时都可以向所有监听者广播

use std::time::Duration;

use serde::Serialize;
use tauri::{AppHandle, Emitter};

// 事件名,前端listen时使用同一个名字
const UPTIME_EVENT: &str = "uptime";

// 事件的payload需实现Serialize + Clone
// rename_all = "camelCase": Rust字段名uptime_secs -> 前端字段名uptimeSecs,符合各自的命名习惯
#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct UptimePayload {
    uptime_secs: u64,
}

// 在lib.rs的setup中调用: 启动后台任务,每秒广播一次应用已运行的秒数
pub fn spawn(app: AppHandle) {
    // 在Tauri内置的async runtime(tokio)中启动异步任务,不阻塞setup
    tauri::async_runtime::spawn(async move {
        let mut uptime_secs = 0;
        loop {
            tokio::time::sleep(Duration::from_secs(1)).await;
            uptime_secs += 1;
            // emit方法来自Emitter trait,需要use tauri::Emitter
            if app
                .emit(UPTIME_EVENT, UptimePayload { uptime_secs })
                .is_err()
            {
                break;
            }
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test01_serialize() {
        let json = serde_json::to_string(&UptimePayload { uptime_secs: 3 }).unwrap();
        println!("{}", json); // print->{"uptimeSecs":3}
        assert_eq!(json, r#"{"uptimeSecs":3}"#);
    }
}
