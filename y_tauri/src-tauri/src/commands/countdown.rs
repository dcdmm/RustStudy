use std::time::Duration;

use serde::Serialize;
use tauri::ipc::Channel;

use crate::error::AppResult;

#[derive(Clone, Serialize)]
#[serde(tag = "event", content = "data", rename_all = "camelCase")]
pub enum CountdownEvent {
    Tick { remaining: u32 },
    Finished,
}

#[tauri::command]
pub async fn countdown(total_seconds: u32, on_event: Channel<CountdownEvent>) -> AppResult<()> {
    for remaining in (1..=total_seconds).rev() {
        on_event.send(CountdownEvent::Tick { remaining })?;
        tokio::time::sleep(Duration::from_secs(1)).await;
    }
    on_event.send(CountdownEvent::Finished)?;
    Ok(())
}
