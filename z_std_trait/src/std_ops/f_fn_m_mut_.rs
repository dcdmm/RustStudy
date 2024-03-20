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
fn t1() {
    fn do_twice<'a, F: FnMut(&'a str) -> ()>(mut func: F) { // 
        func("hello ");
        func("world");
    }

    let mut s1 = String::new();
    let update_string1 = |pstr| s1.push_str(pstr);
    do_twice(update_string1);
    println!("{}", s1); // print->hello world
}

#[test]
fn t2() {
}
