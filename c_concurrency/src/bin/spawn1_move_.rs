use std::thread;

fn main() {
    let v = vec![1, 2, 3];

    // 报错:error[E0373]: closure may outlive the current function, but it borrows `v`, which is owned by the current function
    // let handle = thread::spawn(|| {
    //     println!("Here's a vector: {:?}", v);
    // });
    //
    // 解决方式如下:
    // Using the move keyword to force a closure to take ownership of the values it uses
    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
    });

    handle.join().unwrap();
}