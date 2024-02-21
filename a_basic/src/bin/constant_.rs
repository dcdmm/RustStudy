// 常量

fn main() {
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3; // 常量的定义(通过`const`关键字)
    println!("{}", THREE_HOURS_IN_SECONDS);

    // Like immutable variables, constants are values that are bound to a name and are not allowed to change
    // error[E0428]: the name `THREE_HOURS_IN_SECONDS` is defined multiple times
    // note: `THREE_HOURS_IN_SECONDS` must be defined only once in the value namespace of this block
    // const THREE_HOURS_IN_SECONDS: u32 = 410;

    #[allow(warnings)]
    let i: i32 = 60 * 60 * 3;
    let i: i32 = 410;
    println!("{}", i);
}
