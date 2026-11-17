// 参考文档: https://tauri.app/develop/calling-rust/#error-handling

use std::fmt::Display;

use serde::{Serialize, Serializer};

// 应用统一的错误类型
// 1. thiserror自动实现std::error::Error和Display,#[error("...")]就是Display输出的内容
// 2. command返回Err时,Tauri会把错误序列化后传给前端,所以还需要实现Serialize
#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("除数不能为0")]
    DivideByZero,

    // 前端传来的参数不合法
    #[error("参数错误: {0}")]
    InvalidInput(String),

    // 交给rayon的计算任务没有返回结果就结束了
    #[error("后台计算任务异常结束")]
    TaskAborted,

    // #[from]: 自动实现From<tauri::Error>,在command中可以直接用?转换
    // #[error(transparent)]: Display直接使用tauri::Error的内容
    #[error(transparent)]
    Tauri(#[from] tauri::Error),

    #[error(transparent)]
    ThreadPool(#[from] rayon::ThreadPoolBuildError),
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

// 检查参数是否在[min, max]范围内
// 前端的输入不可信,计算量、数据量等参数必须在Rust端限制,防止长时间卡住或内存耗尽
pub fn ensure_range<T: PartialOrd + Display>(
    name: &str,
    value: T,
    min: T,
    max: T,
) -> AppResult<()> {
    if value < min || value > max {
        return Err(AppError::InvalidInput(format!(
            "{name}必须在{min}到{max}之间,当前为{value}"
        )));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test01_serialize() {
        let json = serde_json::to_string(&AppError::DivideByZero).unwrap();
        println!("{}", json); // print->"除数不能为0"
        assert_eq!(json, r#""除数不能为0""#);
    }

    #[test]
    fn test02_ensure_range() {
        assert!(ensure_range("limit", 5, 1, 10).is_ok());
        let err = ensure_range("limit", 0, 1, 10).unwrap_err();
        println!("{}", err); // print->参数错误: limit必须在1到10之间,当前为0
        assert_eq!(err.to_string(), "参数错误: limit必须在1到10之间,当前为0");
    }
}
