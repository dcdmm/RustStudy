// 示例4: async command + Channel(把一次command调用的执行进度推送给前端)
// 参考文档: https://tauri.app/develop/calling-frontend/#channels

use std::time::Duration;

use serde::Serialize;
use tauri::ipc::Channel;

use crate::error::AppResult;

// 通过Channel发送的消息,需实现Serialize
// #[serde(tag = "event", content = "data")]把枚举序列化为前端容易区分的格式:
// 1. Tick { remaining: 3 } -> {"event":"tick","data":{"remaining":3}}
// 2. Finished              -> {"event":"finished"}
#[derive(Clone, Serialize)]
#[serde(tag = "event", content = "data", rename_all = "camelCase")]
pub enum CountdownEvent {
    Tick { remaining: u32 },
    Finished,
}

// 1. async fn: 在Tauri的async runtime(tokio)中执行,不会卡住界面,耗时操作推荐用async command
// 2. Channel<T>: 前端创建Channel对象作为参数传入,Rust通过它多次send消息,保证顺序且速度快
// 3. Rust参数名在前端要写成camelCase: total_seconds -> totalSeconds, on_event -> onEvent
// 4. 倒计时全部结束后command才返回,前端await的Promise才resolve
#[tauri::command]
pub async fn countdown(total_seconds: u32, on_event: Channel<CountdownEvent>) -> AppResult<()> {
    for remaining in (1..=total_seconds).rev() {
        // send失败时返回tauri::Error,?通过#[from]自动转换为AppError
        on_event.send(CountdownEvent::Tick { remaining })?;
        tokio::time::sleep(Duration::from_secs(1)).await;
    }
    on_event.send(CountdownEvent::Finished)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    // 查看前端收到的JSON格式
    #[test]
    fn test01_serialize() {
        let tick = serde_json::to_string(&CountdownEvent::Tick { remaining: 3 }).unwrap();
        println!("{}", tick); // print->{"event":"tick","data":{"remaining":3}}
        assert_eq!(tick, r#"{"event":"tick","data":{"remaining":3}}"#);

        let finished = serde_json::to_string(&CountdownEvent::Finished).unwrap();
        println!("{}", finished); // print->{"event":"finished"}
        assert_eq!(finished, r#"{"event":"finished"}"#);
    }
}
