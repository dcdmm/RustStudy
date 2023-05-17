// Creating a New Thread with spawn

// the Rust standard library uses a 1:1 model of thread implementation, whereby a program uses one operating system thread per one language thread.

use std::thread;
use std::time::Duration;

fn main() {
    thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    // Note that when the main thread of a Rust program completes, all spawned threads are shut down, whether or not they have finished running.
    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
}