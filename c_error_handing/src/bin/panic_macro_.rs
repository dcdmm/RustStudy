// Unrecoverable Errors with panic!

#[test]
fn t0() {
    panic!("crash and burn"); // 显式调用panic!宏(主动触发,类似使用raise关键字主动触发异常)
}
fn main() {
    let v = vec![1, 2, 3];
    v[99]; // 数组越界访问
}
