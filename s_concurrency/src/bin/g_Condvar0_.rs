#![allow(non_snake_case)]

fn main() {
    use std::sync::{Arc, Condvar, Mutex};
    use std::thread;

    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let pair2 = Arc::clone(&pair);

    thread::spawn(move || {
        let (lock, cvar) = &*pair2;
        let mut started = lock.lock().unwrap();
        *started = true;
        // Wakes up one blocked thread on this condvar.
        cvar.notify_one();
    });

    let (lock, cvar) = &*pair;
    let mut started = lock.lock().unwrap();
    while !*started {
        // Blocks the current thread until this condition variable receives a notification.
        started = cvar.wait(started).unwrap();
    }
}
