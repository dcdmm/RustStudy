use serde::{Serialize, Serializer};

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("路径不能为空")]
    EmptyPath,
    #[error("读取文件失败: {0}")]
    Io(#[from] std::io::Error),
}

// 出错时传给前端的数据,经JSON转换后即为前端catch中的e
#[derive(Serialize)]
struct ErrorBody {
    kind: &'static str,
    message: String,
}

// Err须实现Serialize,Tauri才能将其转为JSON传给前端
// AppError包含的std::io::Error未实现Serialize,无法derive,因此需手动实现
impl Serialize for AppError {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        // 各变体对应的字符串须与前端AppError中kind的取值一致
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

// 返回Ok时invoke得到String;返回Err时invoke抛出ErrorBody,前端进入catch
#[tauri::command]
pub fn read_text(path: String) -> Result<String, AppError> {
    if path.trim().is_empty() {
        return Err(AppError::EmptyPath);
    }
    let content = std::fs::read_to_string(path)?;
    Ok(content)
}
