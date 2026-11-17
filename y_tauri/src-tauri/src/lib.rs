// 参考文档: https://tauri.app/develop/calling-rust/
//
// 本文件包含4个示例,每个示例对应前端src/demos/下的一个组件:
// 1. greet                -> Greet.tsx      command基础: 参数和返回值
// 2. divide               -> Divide.tsx     command错误处理: 返回Result
// 3. get_count/increment  -> Counter.tsx    状态管理: State
// 4. start_countdown      -> Countdown.tsx  事件: Rust主动向前端推送消息

use std::sync::Mutex;
use std::thread;
use std::time::Duration;

use serde::Serialize;
use tauri::{AppHandle, Emitter, State};

// ==================== 示例1: command基础 ====================

// #[tauri::command]把普通函数标记为command,前端通过invoke("greet", { name: "xxx" })调用
// 1. 参数需实现Deserialize(前端传来的JSON -> Rust类型)
// 2. 返回值需实现Serialize(Rust类型 -> JSON -> 前端)
#[tauri::command]
fn greet(name: &str) -> String {
    format!("你好, {}! 这条消息来自Rust", name)
}

// ==================== 示例2: command错误处理 ====================

// 返回Result时:
// 1. Ok(v)  -> 前端invoke返回的Promise resolve,await得到v
// 2. Err(e) -> 前端invoke返回的Promise reject,catch中得到e(e也需实现Serialize)
#[tauri::command]
fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        return Err("除数不能为0".to_string());
    }
    Ok(a / b)
}

// ==================== 示例3: 状态管理 ====================

// 交给Tauri托管的状态,在run()中通过.manage()注册,整个应用共享一份
// State<'_, T>只能拿到&T,要修改数据需要Mutex(内部可变性 + 线程安全)
#[derive(Default)]
struct Counter {
    count: Mutex<u32>,
}

// 参数中声明State<'_, T>,Tauri会自动注入托管的状态,前端调用时不需要传这个参数
#[tauri::command]
fn get_count(counter: State<'_, Counter>) -> u32 {
    *counter.count.lock().unwrap()
}

#[tauri::command]
fn increment(counter: State<'_, Counter>) -> u32 {
    let mut count = counter.count.lock().unwrap();
    *count += 1;
    *count
}

// ==================== 示例4: 事件 ====================

// 事件的payload需实现Serialize + Clone
#[derive(Clone, Serialize)]
struct CountdownPayload {
    remaining: u32,
}

// 1. AppHandle和State一样由Tauri自动注入,前端不需要传
// 2. Rust参数名total_seconds在前端要写成camelCase: invoke("start_countdown", { totalSeconds: 5 })
// 3. 普通(非async)command在主线程执行,直接sleep会卡住界面,所以放到新线程中
// 4. command立即返回,之后每秒通过emit向前端推送一次事件
#[tauri::command]
fn start_countdown(app: AppHandle, total_seconds: u32) {
    thread::spawn(move || {
        for remaining in (0..=total_seconds).rev() {
            // emit方法来自Emitter trait,需要use tauri::Emitter
            app.emit("countdown", CountdownPayload { remaining }).unwrap();
            if remaining > 0 {
                thread::sleep(Duration::from_secs(1));
            }
        }
    });
}

// ==================== 程序入口 ====================

// 桌面端由main.rs调用;移动端由mobile_entry_point宏生成入口
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        // 注册托管状态
        .manage(Counter::default())
        // 注册command,没有注册的command前端调用时会报错: Command xxx not found
        .invoke_handler(tauri::generate_handler![
            greet,
            divide,
            get_count,
            increment,
            start_countdown
        ])
        // generate_context!在编译期读取tauri.conf.json
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[cfg(test)]
mod tests {
    use super::*;

    // command本质上就是普通函数,可以直接写单元测试
    #[test]
    fn test01_greet() {
        let msg = greet("Tauri");
        println!("{}", msg); // print->你好, Tauri! 这条消息来自Rust
        assert_eq!(msg, "你好, Tauri! 这条消息来自Rust");
    }

    #[test]
    fn test02_divide() {
        assert_eq!(divide(6.0, 3.0), Ok(2.0));
        assert_eq!(divide(1.0, 0.0), Err("除数不能为0".to_string()));
    }
}
