mod a_base;
mod b_state;
mod c_async;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        // 启动时创建Counter交给Tauri保管,全局只有一份
        // command参数写State<Counter>即可借用,按类型查找
        .manage(b_state::Counter::default())
        // 登记前端可调用的command
        .invoke_handler(tauri::generate_handler![
            a_base::greet,
            b_state::get_count,
            b_state::increment,
            c_async::slow_sync,
            c_async::slow_async
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
