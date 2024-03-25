// 实现了闭包类型(Fn、FnMut、FnOnce)的参数/返回值

#[test]
fn t0() {
    // 实现了闭包类型(Fn、FnMut、FnOnce)的参数参考:f_advanced/function_pointers_.rs

    fn make_adder_static(x: i32) -> impl Fn(i32) -> i32 { // 实现了特定trait(即Fn(i32) -> i32)的返回值
        move |y| x + y
    }

    fn make_adder_dyn(x: i32) -> Box<dyn Fn(i32) -> i32> { // 实现了特定trait(即Fn(i32) -> i32)的返回值(boxed trait object)
        Box::new(move |y| x + y)
    }

    let add_five_static = make_adder_static(5);
    println!("5 + 3 = {}", add_five_static(3));
    let add_five_dyn = make_adder_dyn(5);
    println!("5 + 3 = {}", add_five_dyn(3));
}
