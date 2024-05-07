#![allow(non_snake_case)]

// Using Mutexes to Allow Access to Data from One Thread at a Time

/*
Mutex is an abbreviation for mutual exclusion, as in, a mutex allows only one thread to access some data at any given time. To access the data in a mutex, a thread must first signal that it wants access by asking to acquire the mutex’s lock. The lock is a data structure that is part of the mutex that keeps track of who currently has exclusive access to the data. Therefore, the mutex is described as guarding the data it holds via the locking system.

Mutexes have a reputation for being difficult to use because you have to remember two rules:
    * You must attempt to acquire the lock before using the data.
    * When you’re done with the data that the mutex guards, you must unlock the data so other threads can acquire the lock.
 */

use std::sync::Mutex;

fn main() {
    // Creates a new mutex in an unlocked state ready for use.
    let m = Mutex::new(5);

    {
        /*
        pub fn lock(&self) -> LockResult<MutexGuard<'_, T>>
            Acquires a mutex, blocking the current thread until it is able to do so.

            # Errors
            If another user of this mutex panicked while holding the mutex, then this call will return an error once the mutex is acquired.

            # Panics
            This function might panic when called if the lock is already held by the current thread.
         */
        let mut num = m.lock().unwrap();
        *num = 6;
    }

    println!("{:?}", m)
}
