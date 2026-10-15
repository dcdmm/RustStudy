// Creating a New Thread with spawn
// C++参考: CPPStudy\i_concurrency\base\constructor_join0_.cpp

// the Rust standard library uses a 1:1 model of thread implementation, whereby a program uses one operating system thread per one language thread.

use std::thread;
use std::time::{Duration, Instant};

fn simulate_task(name: &str, seconds: u64) {
    println!("{name}：开始，预计耗时 {seconds} 秒");
    thread::sleep(Duration::from_secs(seconds));
    println!("{name}：完成");
}

fn main() {
    println!("=== 1. 顺序执行：预计约 5 秒 ===");
    let start = Instant::now();
    simulate_task("任务 A（主线程）", 2);
    simulate_task("任务 B（主线程）", 3);
    println!("顺序执行总耗时：{:.2} 秒", start.elapsed().as_secs_f64());

    println!("\n=== 2. spawn并发执行：预计约 3 秒 ===");
    let start = Instant::now();
    /*
    Spawns a new thread, returning a JoinHandle for it.

    spawn 的函数签名如下:
    pub fn spawn<F, T>(f: F) -> JoinHandle<T>
    where
        F: FnOnce() -> T,
        F: Send + 'static,
        T: Send + 'static
     */
    thread::spawn(|| {
        simulate_task("任务 A（子线程）", 2); 
    });
    simulate_task("任务 B（主线程）", 3);
    println!(
        "主线程完成任务 B，耗时：{:.2} 秒",
        start.elapsed().as_secs_f64()
    );

    println!("\n=== 3. 主线程先结束：预计约 2 秒 ===");
    let start = Instant::now();
    thread::spawn(|| {simulate_task("任务 C（子线程）", 5);});
    simulate_task("任务 D（主线程）", 2);
    // Note that when the main thread of a Rust program completes, all spawned threads are shut down, whether or not they have finished running
    println!("主线程完成任务 D，耗时：{:.2} 秒", start.elapsed().as_secs_f64());
}
