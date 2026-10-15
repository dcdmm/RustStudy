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

#[test]
fn t0() {
    use std::sync::Arc;
    use std::sync::atomic::{AtomicI32, Ordering};
    use std::thread;
    use std::time::Duration;

    println!("=== 1. 没有Mutex:两个线程各加1,却丢失了一次更新 ===");

    // Rust 不允许多个线程直接修改普通共享整数,这里用 AtomicI32 做安全的对比
    // load 和 store 分别保证单次读写安全,但不会把整个加法操作保护起来
    let counter = Arc::new(AtomicI32::new(0));
    let old = counter.load(Ordering::SeqCst);
    println!("主线程:读取到{old},暂时不写回");

    let child_counter = Arc::clone(&counter);
    let handle = thread::spawn(move || {
        let old = child_counter.load(Ordering::SeqCst);
        child_counter.store(old + 1, Ordering::SeqCst);
        println!("子线程:读取到{old},加1后写入{}", old + 1);
    });

    // 特意让子线程先写完,再让主线程用旧值写回,稳定重现更新丢失
    handle.join().unwrap();
    counter.store(old + 1, Ordering::SeqCst);
    println!("主线程:仍根据旧值{old},加1后写入{}", old + 1);
    let result = counter.load(Ordering::SeqCst);
    println!("最终计数:{result},本来应该是2\n");
    assert_eq!(result, 1);

    println!("=== 2. 使用Mutex:保护从读取到写回的完整过程 ===");

    // Arc::clone 让两个线程使用同一个计数器,不会复制内部的数字
    let counter = Arc::new(Mutex::new(0));
    let child_counter = Arc::clone(&counter);
    let handle = thread::spawn(move || {
        println!("子线程:准备加锁");
        let mut num = child_counter.lock().unwrap();
        println!("子线程:拿到锁,读取到{},模拟处理2秒", *num);
        thread::sleep(Duration::from_secs(2));
        *num += 1;
        println!("子线程:写入{},即将自动解锁", *num);
    }); // 闭包执行结束时,num 被销毁,自动释放锁

    {
        println!("主线程:准备加锁");
        let mut num = counter.lock().unwrap();
        println!("主线程:拿到锁,读取到{},模拟处理2秒", *num);
        // 为了看清等待过程,演示时持锁休眠 2 秒
        thread::sleep(Duration::from_secs(2));
        *num += 1;
        println!("主线程:写入{},即将自动解锁", *num);
    } // num 被销毁,自动释放锁

    // 先释放主线程持有的锁,再等待子线程完成,避免互相等待
    handle.join().unwrap();
    let result = *counter.lock().unwrap();
    println!("最终计数:{result},两次加1都保留下来了");
    assert_eq!(result, 2);

    // 谁先拿到锁不固定,但后拿到锁的线程会读到前一个线程更新后的值
}
