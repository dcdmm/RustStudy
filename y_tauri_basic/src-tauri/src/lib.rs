// 前端invoke("greet", { name, age })调用,函数名和参数名需一致
#[tauri::command]
fn greet(name: &str, age: u32) -> Result<String, String> {
    if name.trim().is_empty() {
        // 返回Err时invoke抛出异常
        return Err("名字不能为空".to_string());
    }
    if age > 150 {
        return Err(format!("年龄{}不合理", age));
    }
    // 返回Ok时invoke成功
    Ok(format!(
        "你好, {}! 你今年{}岁了! 这个是rust后端生成的问候!",
        name, age
    ))
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        // 登记前端可调用的command
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
