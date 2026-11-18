use serde::Serialize;

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SystemInfo {
    os: &'static str,
    arch: &'static str,
    cpu_count: usize,
}

#[tauri::command]
pub fn system_info() -> SystemInfo {
    SystemInfo {
        os: std::env::consts::OS,
        arch: std::env::consts::ARCH,
        cpu_count: std::thread::available_parallelism().map_or(1, |n| n.get()),
    }
}
