// 闭包

/*
Closures can capture values from their environment in three ways, which directly map to the three ways a function can take a parameter: borrowing immutably, borrowing mutably, and taking ownership. The closure will decide which of these to use based on what the body of the function does with the captured values.
*/

#[test]
fn t0() {
    // let x0 = 4;
    // fn equal_to_x0(z: i32) -> bool {
    //     // error[E0434]: can't capture dynamic environment in a fn item
    //     z == x0
    // }
    // let _is_equal_func = equal_to_x0(4);
    
    let x1 = 4;
    let equal_to_x1 = |z| z == x;
    let _is_equal_closure = equal_to_x1(4);
}

#[test]
fn only_borrows_() {
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    let only_borrows = || println!("From closure: {:?}", list); // 捕获不可变借用

    println!("Before calling closure: {:?}", list);
    only_borrows();
    println!("After calling closure: {:?}", list);
}

#[test]
fn borrows_mutably_() {
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    let mut borrows_mutably = || list.push(7); // 捕获可变借用

    // error[E0502]: cannot borrow `list` as immutable because it is also borrowed as mutable
    // println!("Before calling closure: {:?}", list);
    borrows_mutably();
    println!("After calling closure: {:?}", list);
}

#[test]
fn take_owership_() {
    let list = vec![1, 2, 3]; // `list`: move occurs because `list` has type `Vec<i32>`, which does not implement the `Copy` trait
    let take_owership = move || list; // `move ||`: value moved into closure here
    take_owership();
    // error[E0382]: borrow of moved value: `list`
    // println!("{:?}", list);
}
