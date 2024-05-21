// Trait std::ops::FnOnce

/*
The version of the call operator that takes a by-value receiver.

Instances of FnOnce can be called, but might not be callable multiple times. Because of this, if the only thing known about a type is that it implements FnOnce, it can only be called once.

FnOnce is implemented automatically by closures that might consume captured variables, as well as all types that implement FnMut, e.g., (safe) function pointers (since FnOnce is a supertrait of FnMut).

Since both Fn and FnMut are subtraits of FnOnce, any instance of Fn or FnMut can be used where a FnOnce is expected.

Use FnOnce as a bound when you want to accept a parameter of function-like type and only need to call it once. If you need to call the parameter repeatedly, use FnMut as a bound; if you also need it to not mutate state, use Fn.
 */

#[test]
fn t0() {
    fn consume_with_relish<F>(func: F)
    // `func`: move occurs because `func` has type `F`, which does not implement the `Copy` trait
    where
        F: FnOnce() -> String,
    {
        // `func` consumes its captured variables, so it cannot be run more than once.
        println!("Consumed: {}", func());

        // error[E0382]: use of moved value: `func`
        // println!("Consumed: {}", func());
    }

    let x = String::from("xXx");
    let consume_and_return_x = || x;
    consume_with_relish(consume_and_return_x);
}

#[test]
fn t1() {
    fn consume_with_relish<F>(func: F)
    where
        F: FnOnce() -> String,
    {
        // `func` consumes its captured variables, so it cannot be run more than once.
        println!("Consumed: {}", func());
    }

    let x = String::from("xXx");
    let consume_and_return_x = || x; // 获取作用域中值的所有权
    consume_with_relish(consume_and_return_x);
    // error[E0382]: borrow of moved value: `x`
    // println!("{}", x);
}

#[test]
fn t2() {
    fn consume_with_relish<F: FnOnce() -> String + Copy>(func: F) {
        println!("Consumed: {}", func());
        println!("Consumed: {}", func()); // F实现了Copy trait,调用时使用的是func的拷贝
    }

    let x = String::from("xXx");
    let consume_and_return_x = || x.clone();
    consume_with_relish(consume_and_return_x);
}
