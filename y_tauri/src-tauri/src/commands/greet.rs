// 示例1: command基础(参数和返回值)
// 参考文档: https://tauri.app/develop/calling-rust/

// #[tauri::command]把普通函数标记为command,前端通过invoke("greet", { name: "xxx" })调用
// 1. 参数需实现Deserialize(前端传来的JSON -> Rust类型)
// 2. 返回值需实现Serialize(Rust类型 -> JSON -> 前端)
#[tauri::command]
pub fn greet(name: &str) -> String {
    format!("你好, {}! 这条消息来自Rust", name)
}

#[cfg(test)]
mod tests {
    use super::*;

    // command本质上就是普通函数,可以直接写单元测试
    #[test]
    fn test01_greet() {
        let msg = greet("Tauri");
        println!("{}", msg); // print->你好, Tauri! 这条消息来自Rust
        assert_eq!(msg, "你好, Tauri! 这条消息来自Rust");
    }
}
