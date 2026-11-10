// 自定义错误类型
// Trait std::error::Error

/*
Error is a trait representing the basic expectations for error values, i.e., values of type E in Result<T, E>.

Errors must describe themselves through the Display and Debug traits. Error messages are typically concise lowercase sentences without trailing punctuation.
*/

use std::error::Error;
use std::fmt;
use std::fs;
use std::io;
use std::num::ParseIntError;

// (必须)实现Debug trait
#[derive(Debug)]
enum AgeError {
    // 一个变体对应一种错误
    Parse(ParseIntError), // 标准库错误
    Io(io::Error),        // 标准库错误
    OutOfRange(u32),      // 自定义错误
    Empty,                // 自定义错误
}

// (必须)实现Display trait
impl fmt::Display for AgeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AgeError::Parse(e) => write!(f, "解析失败: {e}"),
            AgeError::Io(e) => write!(f, "读取文件失败: {e}"),
            AgeError::OutOfRange(n) => write!(f, "年龄超过范围: {n}"),
            AgeError::Empty => write!(f, "输入为空"),
        }
    }
}

// 3. Error: 空实现即可,实现后可被?转换为Box<dyn Error>
impl Error for AgeError {}

// 4. From: 使?能自动将ParseIntError转换为AgeError
impl From<ParseIntError> for AgeError {
    fn from(e: ParseIntError) -> Self {
        AgeError::Parse(e)
    }
}

// 每种需要?转换的标准库错误各实现一个From
impl From<io::Error> for AgeError {
    fn from(e: io::Error) -> Self {
        AgeError::Io(e)
    }
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
