//! # My Crate
//!
//! `my_crate` is a collection of utilities to make performing certain
//! calculations more convenient.

/// x的值加1,并返回
/// 
/// # Note
/// 
/// x的值必须为i32类型
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
pub fn add_one(x: i32) -> i32 {
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

/// other_module_模块
pub mod other_module_;

#[test]
fn xf() {
    use crate::other_module_::public_struct;

    let x = public_struct::Breakfast::summer("rust base");
    let x_bool = x.startwiths_rust();
    assert_eq!(x_bool, true);
}