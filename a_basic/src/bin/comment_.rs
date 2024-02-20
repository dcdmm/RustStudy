//! 计算一些你口算算不出来的复杂算术题


/// `add_one`将指定值加1
///


// 注释

/// 文档注释
///
/// # Panics
///
/// Panics if `index > len`.
///
/// # Examples
///
/// ```
/// let mut vec = Vec::with_capacity(10);
/// vec.extend([1, 2, 3]);
/// assert!(vec.capacity() >= 10);
/// vec.shrink_to_fit();
/// assert!(vec.capacity() >= 3);
/// ```
///
/// * `length` needs to be less than or equal to `capacity`.
/// * The first `length` values must be properly initialized values of type `T`.
/// * `capacity` needs to [*fit*] the layout size that the pointer was allocated with.
/// * The allocated size in bytes must be no larger than `isize::MAX`.
fn main() {
    // 单行注释

    /*
    多行注释
    多行注释
    多行注释
    */
    println!("hello rust!") // 单行注释
}


/// `add_one` 将指定值加1
///
/// # Examples11
///
/// ```
/// let arg = 5;
/// let answer = world_hello::compute::add_one(arg);
///
/// assert_eq!(6, answer);
/// ```
pub fn add_one(x: i32) -> i32 {
    x + 1
}