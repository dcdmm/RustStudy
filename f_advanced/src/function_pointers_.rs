// Function Pointers

/*
Unlike closures, fn is a type rather than a trait, so we specify fn as the parameter type directly rather than declaring a generic type parameter with one of the Fn traits as a trait bound.

Function pointers implement all three of the closure traits (Fn, FnMut, and FnOnce), meaning you can always pass a function pointer as an argument for a function that expects a closure.
It’s best to write functions using a generic type and one of the closure traits so your functions can accept either functions or closures.
 */

#[allow(warnings)]
fn add_one(x: i32) -> i32 {
    x + 1
}

#[allow(warnings)]
fn subtract_one(x: i32) -> i32 {
    x - 1
}

#[allow(warnings)]
fn do_twice_fn(f: fn(i32) -> i32, arg: i32) -> i32 { // 参数为函数指针
    f(arg) + f(arg)
}

#[allow(warnings)]
fn do_twice_f_fn<F>(f: F, arg: i32) -> i32
where
    F: Fn(i32) -> i32,  // 参数实现了特定的triat(即Fn(i32) -> i32)
{
    f(arg) + f(arg)
}

#[allow(warnings)]
fn select_function(add: bool) -> fn(i32) -> i32 { // 返回值为函数指针
    if add {
        add_one
    } else {
        subtract_one
    }
}

#[test]
fn t0() {
    let af: fn(fn(i32) -> i32, i32) -> i32 = do_twice_fn;
    println!("{}", af(add_one, 5));
    println!("{}", af(|x| x + 1, 5));
    println!("{}", do_twice_f_fn(add_one, 5));
    println!("{}", do_twice_f_fn(|x| x + 1, 5));
}

#[test]
fn t1() {
    let s_add: fn(i32) -> i32 = select_function(true);
    println!("{}", s_add(5));
    let s_sub = select_function(false);
    println!("{}", s_sub(5))
}

#[test]
fn t2() {
    #[allow(warnings)]
    enum Status {
        Value(u32),
        Stop,
    }

    let sv: fn(u32) -> Status = Status::Value;
    // Here we create Status::Value instances using each u32 value in the range that map is called on by using the initializer function of Status::Value.
    let _list_of_statuses: Vec<Status> = (0u32..20).map(sv).collect();
}
