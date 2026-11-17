// 应用入口: 只负责把各模块组装起来,具体逻辑都在各自的模块中
//
// Rust模块与前端feature(src/features/)的对应关系,编号与左侧菜单一致:
// 0. commands/system.rs     -> home/            首页: command返回结构体
// 1. commands/greet.rs      -> command/         command基础: 参数和返回值
// 2. commands/math.rs       -> error-handling/  command错误处理: 自定义错误类型
// 3. commands/counter.rs    -> state/           状态管理: State
// 4. commands/countdown.rs  -> channel/         async command + Channel: 推送一次调用的进度
// 5. uptime.rs              -> event/           事件: Rust主动向前端广播
// 6. 无(前端直接调用)        -> window/          窗口API + 权限(capabilities)
// 7. tauri_plugin_opener    -> plugin/          插件

mod commands;
mod error;
mod uptime;

use commands::counter::Counter;

// 桌面端由main.rs调用;移动端由mobile_entry_point宏生成入口
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        // 注册插件,由`npm run tauri add opener`自动添加
        .plugin(tauri_plugin_opener::init())
        // 注册托管状态,command中通过State<'_, Counter>获取
        .manage(Counter::default())
        // 应用启动完成、窗口创建之前执行一次,适合做初始化和启动后台任务
        .setup(|app| {
            uptime::spawn(app.handle().clone());
            Ok(())
        })
        // 注册command,没有注册的command前端调用时会报错: Command xxx not found
        .invoke_handler(tauri::generate_handler![
            commands::system::system_info,
            commands::greet::greet,
            commands::math::divide,
            commands::counter::get_count,
            commands::counter::increment,
            commands::countdown::countdown,
        ])
        // generate_context!在编译期读取tauri.conf.json
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
