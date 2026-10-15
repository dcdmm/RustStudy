#![allow(non_snake_case)]

// Using Mutexes to Allow Access to Data from One Thread at a Time
// C++参考: CPPStudy\i_concurrency\mutex_.cpp

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
    println!("{:?}", m); // print->Mutex { data: 5, poisoned: false, .. }

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
        *num = 6; // MutexGuard是一个智能指针(实现了Deref、DerefMut、Drop特质) 

        println!("{:?}", m); // print->Mutex { data: <locked>, poisoned: false, .. }
    } // 离开作用域时,num被自动丢弃(Drop特质),锁将被解锁

    println!("{:?}", m) // print->Mutex { data: 6, poisoned: false, .. }
}

#[cfg(test)]
fn print_report(name: &str) {
    use std::thread;
    use std::time::Duration;

    println!("报告{name}:第1行");
    thread::sleep(Duration::from_secs(1));
    println!("报告{name}:第2行");
    thread::sleep(Duration::from_secs(1));
    println!("报告{name}:第3行");
}

#[test]
fn t0() {
    use std::thread;

    let handle = thread::spawn(|| print_report("A"));
    print_report("B");

    handle.join().unwrap();
}

#[test]
fn t1() {
    use std::sync::Arc;
    use std::thread;

    // 两个线程共用同一把锁
    let printer = Arc::new(Mutex::new(()));
    let child_printer = Arc::clone(&printer);

    let handle = thread::spawn(move || {
        let _guard = child_printer.lock().unwrap(); // 拿不到锁就等待
        print_report("A");
    }); // 整份报告打印完后,自动解锁

    {
        let _guard = printer.lock().unwrap();
        print_report("B");
    } 

    handle.join().unwrap();
}
