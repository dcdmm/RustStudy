use std::time::Duration;

use serde::Serialize;
use tauri::{AppHandle, Emitter};

const UPTIME_EVENT: &str = "uptime";

#[derive(Clone, Serialize)]
#[serde(rename_all = "camelCase")]
struct UptimePayload {
    uptime_secs: u64,
}

pub fn spawn(app: AppHandle) {
    tauri::async_runtime::spawn(async move {
        let mut uptime_secs = 0;
        loop {
            tokio::time::sleep(Duration::from_secs(1)).await;
            uptime_secs += 1;
            if app
                .emit(UPTIME_EVENT, UptimePayload { uptime_secs })
                .is_err()
            {
                break;
            }
        }
    });
}
