// Capturing References or Moving Ownership

/*
Closures can capture values from their environment in three ways, which directly map to the three ways a function can take a parameter: borrowing immutably, borrowing mutably, and taking ownership. The closure will decide which of these to use based on what the body of the function does with the captured values.

The way a closure captures and handles values from the environment affects which traits the closure implements, and traits are how functions and structs can specify what kinds of closures they can use. Closures will automatically implement one, two, or all three of these Fn traits, in an additive fashion, depending on how the closure’s body handles the values:
* FnOnce applies to closures that can be called once. All closures implement at least this trait, because all closures can be called. A closure that moves captured values out of its body will only implement FnOnce and none of the other Fn traits, because it can only be called once.
* FnMut applies to closures that don’t move captured values out of their body, but that might mutate the captured values. These closures can be called more than once.
* Fn applies to closures that don’t move captured values out of their body and that don’t mutate captured values, as well as closures that capture nothing from their environment. These closures can be called more than once without mutating their environment, which is important in cases such as calling a closure multiple times concurrently.
*/

#[test]
fn only_borrows_() {
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    let only_borrows = || println!("From closure: {:?}", list); // 捕获不可变借用(实现Fn trait)

    println!("Before calling closure: {:?}", list);
    only_borrows();
    println!("After calling closure: {:?}", list);
}

#[test]
fn borrows_mutably_() {
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    let mut borrows_mutably = || list.push(7); // 捕获可变借用(实现FnMut trait)

    // error[E0502]: cannot borrow `list` as immutable because it is also borrowed as mutable
    // println!("Before calling closure: {:?}", list);
    borrows_mutably();
    println!("After calling closure: {:?}", list);
}

#[test]
fn take_owership_() {
    let list = vec![1, 2, 3]; // `list`: move occurs because `list` has type `Vec<i32>`, which does not implement the `Copy` trait
    // move converts any variables captured by reference or mutable reference to variables captured by value.
    let take_owership = move || list; // 获取所有权(实现FnOnce trait);`move ||`: value moved into closure here
    take_owership();
    // error[E0382]: borrow of moved value: `list`
    // println!("{:?}", list); // `list`: value borrowed here after move
}
