// Unrecoverable Errors with panic!

#[test]
fn t0() {
    // panicked:crash and burn
    panic!("crash and burn"); // 显式调用panic!宏(主动触发,类似使用raise关键字主动触发异常)
}
fn main() {
    let v = vec![1, 2, 3];
    // panicked:index out of bounds: the len is 3 but the index is 99
    v[99]; // 数组越界访问
}
