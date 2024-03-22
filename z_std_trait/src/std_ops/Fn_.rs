// Trait std::ops::Fn

/*
The version of the call operator that takes an immutable receiver.

Instances of Fn can be called repeatedly without mutating state.

Fn is implemented automatically by closures which only take immutable references to captured variables or don’t capture anything at all, as well as (safe) function pointers (with some caveats, see their documentation for more details). Additionally, for any type F that implements Fn, &F implements Fn, too.

Since both FnMut and FnOnce are supertraits of Fn, any instance of Fn can be used as a parameter where a FnMut or FnOnce is expected.

Use Fn as a bound when you want to accept a parameter of function-like type and need to call it repeatedly and without mutating state (e.g., when calling it concurrently). If you do not need such strict requirements, use FnMut or FnOnce as bounds.
 */

#[test]
fn t0() {
    fn call_with_one<F: Fn(Vec<i32>) -> Vec<i32>>(func: F) {
        println!("{:?}", func(vec![1, 2, 3]))
    }

    let print_vec1 = |x| x;
    call_with_one(print_vec1)
}