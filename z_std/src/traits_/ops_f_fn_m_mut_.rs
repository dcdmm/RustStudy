// Trait std::ops::FnMut

/*
The version of the call operator that takes a mutable receiver.

Instances of FnMut can be called repeatedly and may mutate state.

签名:pub trait FnMut<Args: Tuple>: FnOnce<Args> {
 */

#[test]
fn t0() {
    // Calling a mutably capturing closure
    let mut s = String::new();
    let mut update_string = |x| s.push_str(x);
    update_string("hello");
    println!("{:?}", s);

    // ******************************************************************
    // Using a FnMut parameter
    // 'a ===> 闭包生命周期为'a(隐式推断) ===> FnMut(&'a str)参数生命周期也为'a ===>
    // ===> 保证闭包的生命周期与函数的生命周期匹配,并且在闭包中进行的操作都在函数调用的生命周期范围内
    fn do_twice<'a, F: FnMut(&'a str) -> ()>(mut func: F)
    {
        func("hello");
        func("world");
    }

    let mut s1 = String::new();
    let update_string1 = |pstr| s1.push_str(pstr);
    do_twice(update_string1);
    println!("{}", s1);
}