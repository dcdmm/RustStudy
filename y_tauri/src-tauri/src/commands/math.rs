use crate::error::{AppError, AppResult};

#[tauri::command]
pub fn divide(a: f64, b: f64) -> AppResult<f64> {
    if b == 0.0 {
        return Err(AppError::DivideByZero);
    }
    Ok(a / b)
}
