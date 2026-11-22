use std::sync::Mutex;
use tauri::State;

#[derive(Default)]
pub struct TodoState {
    items: Mutex<Vec<String>>,
}

#[tauri::command]
pub fn list_todos(state: State<TodoState>) -> Vec<String> {
    state.items.lock().unwrap().clone()
}

#[tauri::command]
pub fn add_todo(state: State<TodoState>, text: String) -> Result<Vec<String>, String> {
    if text.trim().is_empty() {
        return Err("内容不能为空".to_string());
    }
    let mut items = state.items.lock().unwrap();
    items.push(text);
    Ok(items.clone())
}

#[tauri::command]
pub fn remove_todo(state: State<TodoState>, item_index: usize) -> Vec<String> {
    let mut items = state.items.lock().unwrap();
    if item_index < items.len() {
        items.remove(item_index);
    }
    items.clone()
}
