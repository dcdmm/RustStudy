// 自定义错误类型
// Trait std::error::Error

/*
Error is a trait representing the basic expectations for error values, i.e., values of type E in Result<T, E>.

Errors must describe themselves through the Display and Debug traits. Error messages are typically concise lowercase sentences without trailing punctuation.

In error types that wrap an underlying error, the underlying error should be either returned by the outer error's Error::source(), or rendered by the outer error's Display implementation, but not both.
*/

// # Provided Methods
/*
fn source(&self) -> Option<&(dyn Error + 'static)>
    Returns the lower-level source of this error, if any.
*/

use std::error::Error;
use std::fmt;
use std::num::ParseIntError;

#[derive(Debug)] // 1. Debug
enum ConfigError {
    Missing(String),        // 自身产生的错误
    BadPort(ParseIntError), // 包装底层错误
}

// 2. Display
impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigError::Missing(key) => write!(f, "missing key: {key}"),
            ConfigError::BadPort(_) => write!(f, "invalid port"), // 底层错误由source返回,这里不再输出
        }
    }
}

// 3. Error
impl Error for ConfigError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            ConfigError::BadPort(e) => Some(e),
            ConfigError::Missing(_) => None,
        }
    }
}

// 4. From: 使?自动转换错误类型
// Result::Err(e) returns Result::Err(From::from(e))
impl From<ParseIntError> for ConfigError {
    fn from(e: ParseIntError) -> Self {
        ConfigError::BadPort(e)
    }
}

fn parse_port(s: &str) -> Result<u16, ConfigError> {
    let Some(v) = s.strip_prefix("port=") else {
        return Err(ConfigError::Missing("port".to_string()));
    };
    let port = v.parse::<u16>()?; // ParseIntError经From转换为ConfigError
    Ok(port)
}

#[test]
fn t0() {
    assert_eq!(parse_port("port=8080").unwrap(), 8080);

    let e = parse_port("host=a").unwrap_err();
    assert_eq!(e.to_string(), "missing key: port");
    assert!(e.source().is_none());

    let e = parse_port("port=abc").unwrap_err();
    assert_eq!(e.to_string(), "invalid port");
    assert_eq!(e.source().unwrap().to_string(), "invalid digit found in string");
}

fn main() {}
