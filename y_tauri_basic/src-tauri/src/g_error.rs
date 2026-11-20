use serde::{Serialize, Serializer};

// 自定义错误类型,每个变体代表一种错误
// thiserror根据#[error]生成错误信息,即to_string()的结果
#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("路径不能为空")]
    EmptyPath,
    // #[from]使?可将std::io::Error自动转换为AppError::Io
    #[error("读取文件失败: {0}")]
    Io(#[from] std::io::Error),
}

// 传给前端的错误结构
#[derive(Serialize)]
struct ErrorBody {
    kind: &'static str,
    message: String,
}

// command返回的错误须实现Serialize;std::io::Error未实现,因此手动实现
impl Serialize for AppError {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        let kind = match self {
            AppError::EmptyPath => "emptyPath",
            AppError::Io(_) => "io",
        };
        ErrorBody {
            kind,
            message: self.to_string(),
        }
        .serialize(serializer)
    }
}

// 读取文本文件的内容
#[tauri::command]
pub fn read_text(path: String) -> Result<String, AppError> {
    if path.trim().is_empty() {
        return Err(AppError::EmptyPath);
    }
    // 读取失败时,?将std::io::Error转换为AppError并返回Err
    let content = std::fs::read_to_string(path)?;
    Ok(content)
}
