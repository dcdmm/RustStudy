// Sharing a Mutex<T> Between Multiple Threads

// use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::thread;

// fn main() {
//     // 报错:error[E0382]: use of moved value: `counter`
//     /*
//     The error message states that the counter value was moved in the previous iteration of the loop.
//     Rust is telling us that we can’t move the ownership of lock counter into multiple threads.
//      */
//     let counter = Mutex::new(0);
//
//     let mut handles = vec![];
//
//     for _ in 0..10 {
//         let handle = thread::spawn(move || {
//             let mut num = counter.lock().unwrap();
//
//             *num += 1;
//         });
//         handles.push(handle);
//     }
//
//     for handle in handles {
//         handle.join().unwrap();
//     }
//
//     println!("Result: {}", *counter.lock().unwrap());
// }

// fn main() {
//     // 报错:error[E0277]: `Rc<Mutex<i32>>` cannot be sent between threads safely
//     // 报错_auxiliary:the trait `Send` is not implemented for `Rc<Mutex<i32>>`
//     /*
//     Unfortunately, Rc<T> is not safe to share across threads. When Rc<T> manages the reference count, it adds to the count for each call to clone and subtracts from the count when each clone is dropped.
//     But it doesn’t use any concurrency primitives to make sure that changes to the count can’t be interrupted by another thread.
//     This could lead to wrong counts—subtle bugs that could in turn lead to memory leaks or a value being dropped before we’re done with it.
//     What we need is a type exactly like Rc<T> but one that makes changes to the reference count in a thread-safe way.
//      */
//     let counter = Rc::new(Mutex::new(0));
//     let counter = Rc::new(Mutex::new(0));
//     let mut handles = vec![];
//
//     for _ in 0..10 {
//         let counter = Rc::clone(&counter);
//         let handle = thread::spawn(move || {
//             let mut num = counter.lock().unwrap();
//
//             *num += 1;
//         });
//         handles.push(handle);
//     }
//
//     for handle in handles {
//         handle.join().unwrap();
//     }
//
//     println!("Result: {}", *counter.lock().unwrap());
// }

// Using an Arc<T> to wrap the Mutex<T> to be able to share ownership across multiple threads
fn main() {
    // A thread-safe reference-counting pointer. ‘Arc’ stands for ‘Atomically Reference Counted’.
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            /*
            You might have noticed that counter is immutable but we could get a mutable reference to the value inside it;
            this means Mutex<T> provides interior mutability, as the Cell family does.
             */
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}