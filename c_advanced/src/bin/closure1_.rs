// 闭包

// Closures can capture values from their environment in three ways, which directly map to the three ways a function can take a parameter: borrowing immutably, borrowing mutably, and taking ownership. 
fn main() {
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    // 闭包拥有一项函数所不具备的特性:捕获作用域中的值
    let only_borrows = || println!("From closure: {:?}", list);

    println!("Before calling closure: {:?}", list);
    only_borrows();
    println!("After calling closure: {:?}", list);

    println!("**************************************");
    let mut list1 = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list1);

    let mut borrows_mutably = || list1.push(7);

    borrows_mutably();
    println!("After calling closure: {:?}", list1);
}