#![allow(non_snake_case)]

// Struct std::sync::Condvar

// C++参考: CPPStudy\i_concurrency\condition_variable_.cpp

use std::collections::VecDeque;
use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use std::time::Duration;

struct SharedState {
    queue: VecDeque<i32>,
    finished: bool, // 生产者是否完成了生产
}

// 生产者函数
fn producer(shared: Arc<(Mutex<SharedState>, Condvar)>, n: i32) {
    let (lock, cvar) = &*shared; // 获取互斥锁和条件变量的引用

    for i in 0..n {
        {
            let mut data = lock.lock().unwrap();
            data.queue.push_back(i);
            println!("Producing {}", i);
        } // 离开作用域时,data被自动丢弃(Drop特质),锁将被解锁
        // Wakes up one blocked thread on this condvar.
        cvar.notify_one();
        thread::sleep(Duration::from_micros(10));
    }

    let mut data = lock.lock().unwrap();
    data.finished = true;
    // Wakes up all blocked threads on this condvar.
    cvar.notify_all(); // 通知所有消费者生产已完成
}

// 消费者函数
fn consumer(shared: Arc<(Mutex<SharedState>, Condvar)>) {
    let (lock, cvar) = &*shared;

    loop {
        let mut data = lock.lock().unwrap();
        while data.queue.is_empty() && !data.finished {
            // Blocks the current thread until this condition variable receives a notification.
            data = cvar.wait(data).unwrap();
        }
        if !data.queue.is_empty() {
            let item = data.queue.pop_front().unwrap();
            println!("Consuming {}", item);
        } else if data.finished {
            break; // 生产者已完成且队列为空
        }
    }
}

fn main() {
    // Condition variables are typically associated with a boolean predicate (a condition) and a mutex.
    let shared = Arc::new((
        Mutex::new(SharedState {
            queue: VecDeque::new(),
            finished: false,
        }),
        // Creates a new condition variable which is ready to be waited on and notified.
        Condvar::new(),
    ));

    let producer_thread = thread::spawn({
        let shared = shared.clone();
        move || {
            producer(shared, 20);
        }
    });
    let consumer_thread = thread::spawn({
        let shared = shared.clone();
        move || {
            consumer(shared);
        }
    });

    producer_thread.join().unwrap();
    consumer_thread.join().unwrap();
}
