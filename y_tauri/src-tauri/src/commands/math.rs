// 示例2: command错误处理(返回自定义错误类型)

use crate::error::{AppError, AppResult};

// 返回Result时:
// 1. Ok(v)  -> 前端invoke返回的Promise resolve,await得到v
// 2. Err(e) -> 前端invoke返回的Promise reject,catch中得到e序列化后的结果(见error.rs)
#[tauri::command]
pub fn divide(a: f64, b: f64) -> AppResult<f64> {
    if b == 0.0 {
        return Err(AppError::DivideByZero);
    }
    Ok(a / b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test01_divide() {
        assert_eq!(divide(10.0, 4.0).unwrap(), 2.5);
    }

    #[test]
    fn test02_divide_by_zero() {
        let err = divide(1.0, 0.0).unwrap_err();
        println!("{}", err); // print->除数不能为0
        assert!(matches!(err, AppError::DivideByZero));
    }
}
