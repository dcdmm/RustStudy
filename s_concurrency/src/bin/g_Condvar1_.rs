#![allow(non_snake_case)]

use std::collections::VecDeque;
use std::sync::{Arc, Condvar, Mutex};
use std::thread;
use std::time::Duration;

struct SharedState {
    queue: VecDeque<i32>,
    finished: bool,
}

// 生产者函数
fn producer(shared: Arc<(Mutex<SharedState>, Condvar)>, n: i32) {
    let (lock, cvar) = &*shared;

    for i in 0..n {
        let mut data = lock.lock().unwrap();
        data.queue.push_back(i);
        println!("Producing {}", i);
        drop(data);
        cvar.notify_one();
        thread::sleep(Duration::from_micros(10));
    }

    let mut data = lock.lock().unwrap();
    data.finished = true;
    cvar.notify_all(); // 通知所有消费者生产已完成
}

// 消费者函数
fn consumer(shared: Arc<(Mutex<SharedState>, Condvar)>) {
    let (lock, cvar) = &*shared;

    loop {
        let mut data = lock.lock().unwrap();
        while data.queue.is_empty() && !data.finished {
            data = cvar.wait(data).unwrap();
        }
        if !data.queue.is_empty() {
            let item = data.queue.pop_front().unwrap();
            println!("Consuming {}", item);
        } else if data.finished {
            break;
        }
    }
}

fn main() {
    let shared = Arc::new((
        Mutex::new(SharedState {
            queue: VecDeque::new(),
            finished: false,
        }),
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
