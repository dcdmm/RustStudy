#![allow(non_snake_case)]
#![allow(warnings)]

// Sharing a Mutex<T> Between Multiple Threads
// C++参考: CPPStudy\i_concurrency\mutex_.cpp

use std::ops::Deref;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::thread;

#[test]
fn error0() {
    // let counter = Mutex::new(0); // `counter`: move occurs because `counter` has type `Mutex<i32>`, which does not implement the `Copy` trait
    // let mut handles = vec![];
    // for _ in 0..10 {
    //     // error[E0382]: use of moved value: `counter`
    //     // `move ||`: value moved into closure here, in previous iteration of loop
    //     let handle = thread::spawn(move || {
    //         let mut num = counter.lock().unwrap();
    //         *num += 1;
    //     });
    //     handles.push(handle);
    // }
    // for handle in handles {
    //     handle.join().unwrap();
    // }
}

#[test]
fn error1() {
    // let counter = Rc::new(Mutex::new(0));
    // let mut handles = vec![];
    // for _ in 0..10 {
    //     let counter = Rc::clone(&counter);
    //     // error[E0277]: `Rc<std::sync::Mutex<i32>>` cannot be sent between threads safely
    //     let handle = thread::spawn(move || {
    //         let mut num = counter.lock().unwrap();
    //         *num += 1;
    //     });
    //     handles.push(handle);      
    // }
    // for handle in handles {
    //     handle.join().unwrap();
    // }
    // println!("Result: {}", *counter.lock().unwrap());
}

// Using an Arc<T> to wrap the Mutex<T> to be able to share ownership across multiple threads
#[test]
fn right() {
    /*
    A thread-safe reference-counting pointer. ‘Arc’ stands for ‘Atomically Reference Counted’.

    The type Arc<T> provides shared ownership of a value of type T, allocated in the heap. Invoking clone on Arc produces a new Arc instance, which points to the same allocation on the heap as the source Arc, while increasing a reference count. When the last Arc pointer to a given allocation is destroyed, the value stored in that allocation (often referred to as “inner value”) is also dropped.


    # Thread Safety
    Unlike Rc<T>, Arc<T> uses atomic operations for its reference counting. This means that it is thread-safe. The disadvantage is that atomic operations are more expensive than ordinary memory accesses. If you are not sharing reference-counted allocations between threads, consider using Rc<T> for lower overhead.


    # Breaking cycles with Weak
    The downgrade method can be used to create a non-owning Weak pointer. A Weak pointer can be upgraded to an Arc, but this will return None if the value stored in the allocation has already been dropped.

    A cycle between Arc pointers will never be deallocated. For this reason, Weak is used to break cycles. For example, a tree could have strong Arc pointers from parent nodes to children, and Weak pointers from children back to their parents.
     */
    let counter = Arc::new(Mutex::new(0)); // Arc是一个智能指针(实现了Deref、Drop特质) 
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        // 创建多个线程
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {:?}", counter); // print->Result: Mutex { data: 10, poisoned: false, .. }
}

fn main() {
}
