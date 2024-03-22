// Trait std::ops::FnMut

/*
The version of the call operator that takes a mutable receiver.

Instances of FnMut can be called repeatedly and may mutate state.

FnMut is implemented automatically by closures which take mutable references to captured variables, as well as all types that implement Fn, e.g., (safe) function pointers (since FnMut is a supertrait of Fn). Additionally, for any type F that implements FnMut, &mut F implements FnMut, too.

Since FnOnce is a supertrait of FnMut, any instance of FnMut can be used where a FnOnce is expected, and since Fn is a subtrait of FnMut, any instance of Fn can be used where FnMut is expected.

Use FnMut as a bound when you want to ccept a parameter of function-like type and need to call it repeatedly, while allowing it to mutate state. If you don’t want the parameter to mutate state, use Fn as a bound; if you don’t need to call it repeatedly, use FnOnce.
 */

#[test]
fn t0() {
    fn do_twice<F>(mut func: F)
    where
        F: FnMut(),
    {
        func();
        func();
    }

    let mut x: usize = 1;
    {
        let add_two_to_x = || x += 2;
        do_twice(add_two_to_x);
    }
    println!("{}", x); // print->5
}

#[test]
fn t10() {
    // 要求闭包参数(即字符串切片的引用)的生命周期至少要和'a一样长(可能过于严格)
    fn do_twice<'a, F: FnMut(&'a str) -> ()>(mut func: F) {
        func("hello "); // String literals的生命周期为static
        func("world");
    }
    let mut s1 = String::new();
    let update_string1 = |pstr| s1.push_str(pstr);
    do_twice(update_string1);
    println!("{}", s1); // print->hello world
}

#[test]
fn t11() {
    // fn do_twice<'a, F: FnMut(&'a str) -> ()>(mut func: F) { // `'a`: lifetime `'a` defined here
    //     let temp_str = String::from("hello "); // `temp_str`: binding `temp_str` declared here
    //     // error[E0597]: `temp_str` does not live long enough
    //     let temp_slice = temp_str.as_str();
    //     func(temp_slice); // `func(temp_slice)`: argument requires that `temp_str` is borrowed for `'a`
    //     func("world");
    // } // `}`: `temp_str` dropped here while still borrowed
    // let mut s1 = String::new();
    // let update_string1 = |pstr| s1.push_str(pstr);
    // do_twice(update_string1);
}

#[test]
fn t12() {
    fn do_twice<F: FnMut(&str) -> ()>(mut func: F) {
        let temp_str = String::from("hello "); // temp_str的作用域为函数do_twice内都
        let temp_slice = temp_str.as_str();
        func(temp_slice); // 闭包调用时s1(捕获变量)和temp_slice都是有效的
        func("world");
    }
    let mut s1 = String::new(); // s1的作用域为函数t12内部
    let update_string1 = |pstr: &str| s1.push_str(pstr); // 编译器推断闭包的生命周期
    do_twice(update_string1);
}

#[test]
fn t13() {
    #[allow(warnings)]
    fn update_string_fn(pstr: &str) -> &str {
        pstr
    }
    // error: lifetime may not live long enough
    // let update_string0 = |pstr: &str| -> &str { pstr }; // 无法确定返回值生命周期(依赖上下文)足够长

    fn do_twice<'a, F: FnMut(&'a str) -> &'a str>(mut func: F) {
        func("world");
    }
    let update_string1 = |pstr| pstr;
    do_twice(update_string1);
}
