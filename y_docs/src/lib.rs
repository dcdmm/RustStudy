//! # 文档注释(描述整个crate)
//!
//! 置于lib.rs文件代码顶部

#![allow(warnings)]

/*
cargo doc [options]: Build a package’s documentation

# ESCRIPTION
Build the documentation for the local package and all dependencies. The output is placed in target/doc in rustdoc’s usual format.

# OPTIONS
--open
    Open the docs in a browser after building them. This will use your default browser unless you define another one in the BROWSER environment variable or use the doc.browser configuration option.
--document-private-items
    Include non-public items in the documentation(默认仅为pub项生成文档). This will be enabled by default if documenting a binary target.
*/

/// x的值加1,并返回
///
/// # Note
///
/// x的值必须为`i32`类型
///
/// # Examples
///
/// ```
/// use crate::add_one;
///
/// let arg = 5;
/// let answer = add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
/// # Panics
///
/// 如果x的值小于-1,将发生恐慌
pub fn add_one(x: i32) -> i32 {
    if x < -1 {
        panic!("!");
    }
    x + 1
}

/// kinds模块
///
/// # Examples
///
/// PrimaryColor枚举的创建
///
/// ```rust
/// use crate::kinds::PrimaryColor;
///
/// let _ = PrimaryColor::Blue;
/// ```
///
/// SecondaryColor枚举的创建
///
/// ```rust
/// use crate::kinds::SecondaryColor;
///
/// let _ = SecondaryColor::Green;
/// ```
pub mod kinds {
    /// kinds模块下的PrimaryColor枚举
    pub enum PrimaryColor {
        Red,
        Yellow,
        Blue,
    }

    /// kinds模块下的SecondaryColor枚举
    pub enum SecondaryColor {
        Orange,
        Green,
        Purple,
    }
}

pub mod other_module_;
