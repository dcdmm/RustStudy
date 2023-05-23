// Waiting for All Threads to Finish Using join Handles

use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }

    /*
    pub fn join(self) -> Result<T>

        Waits for the associated thread to finish.
        If the associated thread panics, Err is returned with the parameter given to panic!.

    # Panics
        This function may panic on some platforms if a thread attempts to join itself or otherwise may create a deadlock with joining threads.
     */
    handle.join().unwrap();
}