// 常量

#[test]
fn t0() {
    // Like immutable variables, constants are values that are bound to a name and are not allowed to change
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3; // 常量的定义(通过`const`关键字)
    println!("{}", THREE_HOURS_IN_SECONDS)
}
