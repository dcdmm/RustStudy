use std::fmt::Display;

use serde::{Serialize, Serializer};

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("除数不能为0")]
    DivideByZero,

    #[error("参数错误: {0}")]
    InvalidInput(String),

    #[error("后台计算任务异常结束")]
    TaskAborted,

    #[error(transparent)]
    Tauri(#[from] tauri::Error),

    #[error(transparent)]
    ThreadPool(#[from] rayon::ThreadPoolBuildError),

    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Json(#[from] serde_json::Error),
}

impl Serialize for AppError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

pub type AppResult<T> = Result<T, AppError>;

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
