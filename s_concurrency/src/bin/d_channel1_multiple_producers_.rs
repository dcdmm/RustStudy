// Creating Multiple Producers by Cloning the Transmitter

use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel();

    /*
    This time, before we create the first spawned thread, we call clone on the transmitter. This will give us a new transmitter we can pass to the first spawned thread. We pass the original transmitter to a second spawned thread. This gives us two threads, each sending different messages to the one receiver.
     */
    let tx1 = tx.clone();
    thread::spawn(move || {
        let vals = vec![
            String::from("1. hi"),
            String::from("1. from"),
            String::from("2. the"),
            String::from("1. thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    thread::spawn(move || {
        let vals = vec![
            String::from("2. more"),
            String::from("2. messages"),
            String::from("2. for"),
            String::from("2. you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}