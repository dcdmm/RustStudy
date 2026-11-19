use std::sync::Mutex;
use tauri::State;

// 由Tauri托管的状态,所有command共享;State只提供&引用,修改需借助Mutex
#[derive(Default)]
pub struct Counter {
    value: Mutex<u32>,
}

// counter由Tauri自动注入,前端无需传入
#[tauri::command]
pub fn get_count(counter: State<Counter>) -> u32 {
    *counter.value.lock().unwrap()
}

#[tauri::command]
pub fn increment(counter: State<Counter>) -> u32 {
    let mut value = counter.value.lock().unwrap();
    *value += 1;
    *value
}
