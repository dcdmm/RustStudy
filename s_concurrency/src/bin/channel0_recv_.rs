// Using Message Passing to Transfer Data Between Threads

/*
One increasingly popular approach to ensuring safe concurrency is message passing, where threads or actors communicate by sending each other messages containing data.
Here’s the idea in a slogan from the Go language documentation(https://golang.org/doc/effective_go.html#concurrency): “Do not communicate by sharing memory; instead, share memory by communicating.”

To accomplish message-sending concurrency, Rust's standard library provides an implementation of channels.
A channel is a general programming concept by which data is sent from one thread to another.
 */

use std::sync::mpsc;
use std::thread;

fn main() {
    /*
    We create a new channel using the mpsc::channel function;
    mpsc stands for multiple producer, single consumer.
    In short, the way Rust’s standard library implements channels means a channel can have multiple sending ends that produce values but only one receiving end that consumes those values.

    The mpsc::channel function returns a tuple, the first element of which is the sending end--the transmitter--and the second element is the receiving end--the receiver.
     */
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        let val = String::from("hi");
        // moving tx to a spawned thread and sending “hi”
        // The send method returns a Result<T, E> type, so if the receiver has already been dropped and there’s nowhere to send a value, the send operation will return an error.
        tx.send(val).unwrap();

        /*
         The send function takes ownership of its parameter, and when the value is moved, the receiver takes ownership of it.
         This stops us from accidentally using the value again after sending it;
         the ownership system checks that everything is okay.
         */
        // 报错:error[E0382]: borrow of moved value: `val`
        // println!("val is {}", val);
    });

    // Receiving the value “hi” in the main thread and printing it
    // Once a value is sent, recv will return it in a Result<T, E>. When the transmitter closes, recv will return an error to signal that no more values will be coming.
    let received = rx.recv().unwrap();
    println!("Got:{}", received)
}

// Buffering behavior:
#[test]
fn t0() {
    use std::sync::mpsc;
    use std::thread;
    use std::sync::mpsc::RecvError;

    let (send, recv) = mpsc::channel();
    let handle = thread::spawn(move || {
        send.send(1u8).unwrap();
        send.send(2).unwrap();
        send.send(3).unwrap();
        drop(send);
    });

    // wait for the thread to join so we ensure the sender is dropped
    handle.join().unwrap();

    assert_eq!(Ok(1), recv.recv());
    assert_eq!(Ok(2), recv.recv());
    assert_eq!(Ok(3), recv.recv());
    assert_eq!(Err(RecvError), recv.recv());
}