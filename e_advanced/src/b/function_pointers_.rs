// Function Pointers(Primitive Type fn)

fn add_one(x: i32) -> i32 {
    x + 1
}

/*
Unlike closures, fn is a type rather than a trait, so we specify fn as the parameter type directly rather than declaring a generic type parameter with one of the Fn traits as a trait bound.

Function pointers implement all three of the closure traits (Fn, FnMut, and FnOnce), meaning you can always pass a function pointer as an argument for a function that expects a closure.
It’s best to write functions using a generic type and one of the closure traits so your functions can accept either functions or closures.
 */
fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
    f(arg) + f(arg)
}

#[test]
fn t0() {
    let answer = do_twice(add_one, 5);

    println!("The answer is: {}", answer);

    #[allow(dead_code)]
    enum Status {
        Value(u32),
        Stop,
    }

    // We can use these initializer functions as function pointers that implement the closure traits, which means we can specify the initializer functions as arguments for methods that take closures, like so:
    let _list_of_statuses: Vec<Status> = (0u32..20).map(Status::Value).collect(); // Here we create Status::Value instances using each u32 value in the range that map is called on by using the initializer function of Status::Value.
}