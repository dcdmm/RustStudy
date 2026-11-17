// 首页: command返回结构体

use serde::Serialize;

// 返回结构体时需实现Serialize,前端拿到的是一个普通的JS对象
// rename_all = "camelCase": Rust字段名cpu_count -> 前端字段名cpuCount
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

#[cfg(test)]
mod tests {
    use super::*;

    // 查看前端收到的JSON格式
    #[test]
    fn test01_serialize() {
        let json = serde_json::to_string(&system_info()).unwrap();
        println!("{}", json); // print->{"os":"windows","arch":"x86_64","cpuCount":16}
        assert!(json.contains(r#""cpuCount":"#));
    }
}
