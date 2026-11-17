// release模式下不额外弹出控制台窗口(仅Windows),不要删除
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// 桌面端入口,真正的逻辑都在lib.rs中(移动端也复用lib.rs)
fn main() {
    y_tauri_lib::run()
}
