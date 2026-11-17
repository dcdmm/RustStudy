// 参考文档: https://tauri.app/develop/calling-rust/#error-handling

use serde::{Serialize, Serializer};

// 应用统一的错误类型
// 1. thiserror自动实现std::error::Error和Display,#[error("...")]就是Display输出的内容
// 2. command返回Err时,Tauri会把错误序列化后传给前端,所以还需要实现Serialize
#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("除数不能为0")]
    DivideByZero,

    // #[from]: 自动实现From<tauri::Error>,在command中可以直接用?转换
    // #[error(transparent)]: Display直接使用tauri::Error的内容
    #[error(transparent)]
    Tauri(#[from] tauri::Error),
}

// 序列化为错误信息字符串,前端catch到的就是这个字符串
impl Serialize for AppError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

// 所有可能失败的command都返回AppResult<T>
pub type AppResult<T> = Result<T, AppError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test01_serialize() {
        let json = serde_json::to_string(&AppError::DivideByZero).unwrap();
        println!("{}", json); // print->"除数不能为0"
        assert_eq!(json, r#""除数不能为0""#);
    }
}
