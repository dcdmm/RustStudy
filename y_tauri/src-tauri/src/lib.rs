mod commands;
mod error;
mod uptime;

use commands::counter::Counter;
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(Counter::default())
        .setup(|app| {
            uptime::spawn(app.handle().clone());
            app.manage(io_tasks::UrlChecker::new()?);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::system::system_info,
            commands::greet::greet,
            commands::math::divide,
            commands::counter::get_count,
            commands::counter::increment,
            commands::countdown::countdown,
            commands::cpu::cpu_threads,
            commands::cpu::count_primes,
            commands::cpu::word_frequency,
            commands::cpu::sort_numbers,
            commands::io::check_urls,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
