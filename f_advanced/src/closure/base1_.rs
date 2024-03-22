// 参数/返回值为闭包类型

#[test]
fn t0() {
    // 参数为闭包类型参考:f_advanced/function_pointers_.rs
    fn make_adder(x: i32) -> Box<dyn Fn(i32) -> i32> { // 返回值为闭包类型
        Box::new(move |y| x + y)
    }

    let add_five: Box<dyn Fn(i32) -> i32> = make_adder(5);
    println!("5 + 3 = {}", add_five(3));
}
