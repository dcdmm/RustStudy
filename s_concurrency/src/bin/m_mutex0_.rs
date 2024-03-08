// Using Mutexes to Allow Access to Data from One Thread at a Time

/*
Mutex is an abbreviation for mutual exclusion, as in, a mutex allows only one thread to access some data at any given time.
To access the data in a mutex, a thread must first signal that it wants access by asking to acquire the mutex’s lock.
The lock is a data structure that is part of the mutex that keeps track of who currently has exclusive access to the data.
Therefore, the mutex is described as guarding the data it holds via the locking system.

Mutexes have a reputation for being difficult to use because you have to remember two rules:
* You must attempt to acquire the lock before using the data.
* When you’re done with the data that the mutex guards, you must unlock the data so other threads can acquire the lock.
 */

use std::sync::Mutex;

fn main() {
    let m = Mutex::new(5);
    {
        /*
        To access the data inside the mutex, we use the lock method to acquire the lock.
        This call will block the current thread so it can’t do any work until it’s our turn to have the lock.

        The call to lock would fail if another thread holding the lock panicked.
        In that case, no one would ever be able to get the lock, so we’ve chosen to unwrap and have this thread panic if we’re in that situation
         */
        let mut num = m.lock().unwrap();
        /*
        After we’ve acquired the lock, we can treat the return value, named num in this case, as a mutable reference to the data inside.
        The type system ensures that we acquire a lock before using the value in m.
        The type of m is Mutex<i32>, not i32, so we must call lock to be able to use the i32 value. We can’t forget; the type system won’t let us access the inner i32 otherwise.

        The MutexGuard smart pointer implements Deref to point at our inner data;
        the smart pointer also has a Drop implementation that releases the lock automatically when a MutexGuard goes out of scope, which happens at the end of the inner scope.
        As a result, we don’t risk forgetting to release the lock and blocking the mutex from being used by other threads, because the lock release happens automatically.
         */
        *num = 6;  //
    }
    println!("m = {:?}", m)
}