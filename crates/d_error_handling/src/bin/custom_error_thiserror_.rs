// 自定义错误类型(thiserror实现)

/*
This library provides a convenient derive macro for the standard library's std::error::Error trait.
*/

use std::fs;
use std::io;
use std::num::ParseIntError;
use thiserror::Error;

// 1. Debug: 仍需derive
// 3. Error: 由#[derive(Error)]生成,替代手写的impl Error for AgeError {}
#[derive(Debug, Error)]
enum AgeError {
    // 2. Display: 由#[error("...")]生成,替代手写的impl fmt::Display,{0}引用第0个字段
    // 4. From: 由#[from]生成,替代手写的impl From<ParseIntError> for AgeError
    #[error("解析失败: {0}")]
    Parse(#[from] ParseIntError), // 标准库错误
    #[error("读取文件失败: {0}")]
    Io(#[from] io::Error), // 标准库错误
    #[error("年龄超过范围: {0}")]
    OutOfRange(u32), // 自定义错误
    #[error("输入为空")]
    Empty, // 自定义错误
}

fn parse_age(s: &str) -> Result<u32, AgeError> {
    if s.is_empty() {
        return Err(AgeError::Empty);
    }
    // 解析失败时,?调用From::from将ParseIntError转换为AgeError
    let age: u32 = s.parse()?;
    if age > 150 {
        return Err(AgeError::OutOfRange(age));
    }
    Ok(age)
}

fn read_age(path: &str) -> Result<u32, AgeError> {
    // 读取失败时,?调用From::from将io::Error转换为AgeError
    let s = fs::read_to_string(path)?;
    parse_age(s.trim())
}

#[test]
fn t0() {
    assert_eq!(parse_age("18").unwrap(), 18);
    assert_eq!(
        parse_age("200").unwrap_err().to_string(),
        "年龄超过范围: 200"
    );
    assert_eq!(
        parse_age("abc").unwrap_err().to_string(),
        "解析失败: invalid digit found in string"
    );
    assert_eq!(parse_age("").unwrap_err().to_string(), "输入为空");
    assert!(matches!(read_age("no_such_file.txt"), Err(AgeError::Io(_))));
}

fn main() {}
