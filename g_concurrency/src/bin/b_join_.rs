#![allow(non_snake_case)]

// Waiting for All Threads to Finish Using join Handles
// C++参考: CPPStudy\i_concurrency\base\constructor_join0_.cpp

use std::thread;
use std::time::{Duration, Instant};

fn main() {
    let handle = thread::spawn(|| {
        println!("子线程: 开始任务,需要3秒");
        thread::sleep(Duration::from_secs(3));
        println!("子线程: 任务完成！");
    });

    println!("主线程: 开始自己的任务,需要1秒");
    thread::sleep(Duration::from_secs(1));
    println!("主线程: 自己的任务完成,调用join等待子线程...");

    let wait_start = Instant::now();
    /*
    pub fn join(self) -> Result<T>

        Waits for the associated thread to finish.
        If the associated thread panics, Err is returned with the parameter given to panic!.

        # Panics
            This function may panic on some platforms if a thread attempts to join itself or otherwise may create a deadlock with joining threads.
     */
    handle.join().unwrap();
    println!("主线程: join等待结束,等待了{:.2}秒",wait_start.elapsed().as_secs_f64());
}
