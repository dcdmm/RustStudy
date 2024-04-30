// Using move Closures with Threads

use std::thread;

fn main() {
    let v = vec![1, 2, 3];

    // error[E0373]: closure may outlive the current function, but it borrows `v`, which is owned by the current function
    // let handle = thread::spawn(|| {
    //     println!("Here's a vector: {:?}", v); // 捕获作用域中值的不可变借用(自动实现Fn trait)
    // }); // Rust can’t tell how long the spawned thread will run, so it doesn’t know if the reference to v will always be valid.
    // 修正如下:
    // *****************************************************************
    // Using the move keyword to force a closure to take ownership of the values it uses
    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });
    // *****************************************************************
    handle.join().unwrap();
}
