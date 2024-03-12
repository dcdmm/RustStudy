// 闭包(FnOnce/FnMut/Fn)

// ======> 参考见:Z_std_trait/src/bin/ops_f_fn_.rs
// ======> 参考见:Z_std_trait/src/bin/ops_f_fn_m_mut_.rs
// ======> 参考见:Z_std_trait/src/bin/ops_f_fno_once_.rs

/*
Once a closure has captured a reference or captured ownership of a value from the environment where the closure is defined (thus affecting what, if anything, is moved into the closure), the code in the body of the closure defines what happens to the references or values when the closure is evaluated later (thus affecting what, if anything, is moved out of the closure).
A closure body can do any of the following: move a captured value out of the closure, mutate the captured value, neither move nor mutate the value, or capture nothing from the environment to begin with.


The way a closure captures and handles values from the environment affects which traits the closure implements, and traits are how functions and structs can specify what kinds of closures they can use.
 Closures will automatically implement one, two, or all three of these Fn traits, in an additive fashion, depending on how the closure’s body handles the values:
1. FnOnce applies to closures that can be called once. All closures implement at least this trait, because all closures can be called. A closure that moves captured values out of its body will only implement FnOnce and none of the other Fn traits, because it can only be called once.
2. FnMut applies to closures that don’t move captured values out of their body, but that might mutate the captured values. These closures can be called more than once.
3. Fn applies to closures that don’t move captured values out of their body and that don’t mutate captured values, as well as closures that capture nothing from their environment. These closures can be called more than once without mutating their environment, which is important in cases such as calling a closure multiple times concurrently.
 */

#[test]
fn t0() {
    // 报错:can't capture dynamic environment in a fn item:
    // let x = 4;
    // fn equal_to_x(z: i32) -> bool {
    //     z == x
    // }
    // let _is_equal_func= equal_to_x(4);

    // Closures can capture values from their environment in three ways, which directly map to the three ways a function can take a parameter: borrowing immutably, borrowing mutably, and taking ownership. The closure will decide which of these to use based on what the body of the function does with the captured values.

    // we define a closure that captures an immutable reference to the vector named list because it only needs an immutable reference to print the value:
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    let only_borrows = || println!("From closure: {:?}", list);

    println!("Before calling closure: {:?}", list);
    only_borrows();
    println!("After calling closure: {:?}", list);

    println!("************************************************************");
    // we change the closure body so that it adds an element to the list vector. The closure now captures a mutable reference:
    let mut list1 = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list1);

    let mut borrows_mutably = || list1.push(7);

    borrows_mutably();
    println!("After calling closure: {:?}", list1);

    println!("************************************************************");
    let x1 = vec![1, 2, 3];
    let fn_once1 = move || x1; // Using move to force the closure for the thread to take ownership of list
    println!("{:?}", fn_once1());
    // 报错:borrow of moved value: `x1`
    // println!("{:?}", x1);
}