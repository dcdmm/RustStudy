// Trait std::ops::Fn

/*

The version of the call operator that takes an immutable receiver.

Instances of Fn can be called repeatedly without mutating state.
 */

fn main() {
    // Calling a closure
    let print_vec = |x| x;
    let pv = print_vec(vec![1, 2, 3]);
    println!("{:?}", pv);

    // ******************************************************************]
    // Using a Fn parameter
    fn call_with_one<F: Fn(Vec<i32>) -> Vec<i32>>(func: F) {
        println!("{:?}", func(vec![1, 2, 3]))
    }

    let print_vec1 = |x| x;
    call_with_one(print_vec1)
}