#![allow(non_snake_case)]

// Struct std::sync::Mutex

// Poisoning
/*
Once a mutex is poisoned, all other threads are unable to access the data by default as it is likely tainted (some invariant is not being upheld). For a mutex, this means that the lock and try_lock methods return a Result which indicates whether a mutex has been poisoned or not. Most usage of a mutex will simply unwrap() these results, propagating panics among threads to ensure that a possibly invalid invariant is not witnessed.

Poisoning is only advisory: the PoisonError type has an into_inner method which will return the guard that would have otherwise been returned on a successful lock. This allows access to the data, despite the lock being poisoned.
*/

use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let mutex = Arc::new(Mutex::new(1));

    let c_mutex = Arc::clone(&mutex);
    let _ = thread::spawn(move || {
        let mut data = c_mutex.lock().unwrap();
        *data = 2;
        panic!();
    })
    .join();

    println!("{:?}", mutex); // print->Mutex { data: 2, poisoned: true, .. }

    // Determines whether the mutex is poisoned.
    println!("{}", mutex.is_poisoned()); // print->true

    match mutex.lock() {
        Ok(_) => unreachable!(),
        Err(p_err) => {
            // Consumes this error indicating that a lock is poisoned, returning the underlying guard to allow access regardless.
            let data = p_err.into_inner();
            println!("recovered: {data}");
        }
    };
}
