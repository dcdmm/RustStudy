mod a_base;
mod b_state;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        // 注册State,command参数写State<Counter>即可取到
        .manage(b_state::Counter::default())
        // 登记前端可调用的command,其他模块中的command需pub并写模块路径
        .invoke_handler(tauri::generate_handler![
            a_base::greet,
            b_state::get_count,
            b_state::increment
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
