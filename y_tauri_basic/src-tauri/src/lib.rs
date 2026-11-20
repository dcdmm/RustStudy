mod a_base;
mod b_state;
mod c_async;
mod d_channel;
mod e_event;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        // 启动时创建Counter交给Tauri保管,全局只有一份
        // command参数写State<Counter>即可借用,按类型查找
        .manage(b_state::Counter::default())
        // 程序启动时执行一次,用于启动后台任务
        // handle()返回引用,clone后才能交给后台线程
        .setup(|app| {
            e_event::start_ticker(app.handle().clone());
            Ok(())
        })
        // 登记前端可调用的command
        .invoke_handler(tauri::generate_handler![
            a_base::greet,
            b_state::get_count,
            b_state::increment,
            c_async::slow_sync,
            c_async::slow_async,
            d_channel::run_task
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
