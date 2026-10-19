// Using Message Passing to Transfer Data Between Threads

/*
One increasingly popular approach to ensuring safe concurrency is message passing, where threads or actors communicate by sending each other messages containing data.

To accomplish message-sending concurrency, Rust's standard library provides an implementation of channels. A channel is a general programming concept by which data is sent from one thread to another.
*/

use std::{sync::mpsc, thread, time::Duration};

fn main() {
    /*
    We create a new channel using the mpsc::channel function; mpsc stands for multiple producer, single consumer. In short, the way Rust’s standard library implements channels means a channel can have multiple sending ends that produce values but only one receiving end that consumes those values.

    The mpsc::channel function returns a tuple, the first element of which is the sending end--the transmitter--and the second element is the receiving end--the receiver.
     */
    let (tx, rx) = mpsc::channel();

    // Moving tx to a spawned thread and sending "hi"
    thread::spawn(move || {
        let val = String::from("hi");
        // Attempts to send a value on this channel, returning it back if it could not be sent.This method will never block the caller in order to wait for space to become available.
        // The send method returns a Result<T, E> type, so if the receiver has already been dropped and there’s nowhere to send a value, the send operation will return an error.
        tx.send(val).unwrap(); // `val`: value moved here

        /*
        The send function takes ownership of its parameter, and when the value is moved, the receiver takes ownership of it. This stops us from accidentally using the value again after sending it; the ownership system checks that everything is okay.
        */
        // error[E0382]: borrow of moved value: `val`
        // println!("val is {}", val);

        thread::sleep(Duration::from_secs(2)); // block for two seconds
        tx.send(String::from("Delayed for 2 seconds")).unwrap();
    });

    /*
    pub fn recv(&self) -> Result<T, RecvError>
        Attempts to wait for a value on this receiver, returning an error if the corresponding channel has hung up.

        This function will always block the current thread if there is no data available and it’s possible for more data to be sent (at least one sender still exists). Once a message is sent to the corresponding Sender (or SyncSender), this receiver will wake up and return that message.

        If the corresponding Sender has disconnected, or it disconnects while this call is blocking, this call will wake up and return Err to indicate that no more messages can ever be received on this channel. However, since channels are buffered, messages sent before the disconnect will still be properly received.
     */
    let received = rx.recv().unwrap();
    println!("{}", received); // Received immediately
    println!("Waiting...");
    println!("{}", rx.recv().unwrap()); // Received after 2 seconds
    println!("{:?}", rx.recv()); // print->Err(RecvError)
}

#[test]
fn iter_test() {
    use std::sync::mpsc::channel;
    use std::thread;

    let (send, recv) = channel();

    thread::spawn(move || {
        send.send(1).unwrap();
        send.send(2).unwrap();
        send.send(3).unwrap();
    });

    // Returns an iterator that will block waiting for messages, but never panic!. It will return None when the channel has hung up.
    let mut iter = recv.iter();
    assert_eq!(iter.next(), Some(1));
    assert_eq!(iter.next(), Some(2));
    assert_eq!(iter.next(), Some(3));
    assert_eq!(iter.next(), None);
}

#[test]
fn test_sync_channel() {
    use std::sync::mpsc::TrySendError;

    // Creates a new synchronous, bounded channel.
    // This channel has an internal buffer on which messages will be queued. bound specifies the buffer size. When the internal buffer becomes full, future sends will block waiting for the buffer to open up.
    let (tx, rx) = mpsc::sync_channel(1);
    let tx1 = tx.clone();
    tx.send(10).unwrap();

    // Attempts to send a value on this channel without blocking.
    // This method differs from send by returning immediately if the channel’s buffer is full or no receiver is waiting to acquire some data. Compared with send, this function has two failure cases instead of one (one for disconnection, one for a full buffer
    assert_eq!(tx.try_send(20), Err(TrySendError::Full(20)));
    println!("缓冲区已有10,尝试发送20时返回Full");

    // 独立线程接收并收集通道消息
    let receiver = thread::spawn(move || {
        let mut received = Vec::new();
        for value in rx {
            println!("收到:{value}");
            received.push(value);
        }
        println!("所有发送端已释放,接收结束");
        received
    });

    // 子线程发送20和30,主线程发送40,跨线程顺序不固定
    let sender = thread::spawn(move || {
        tx.send(20).unwrap();
        tx.send(30).unwrap();
    });
    tx1.send(40).unwrap();

    // 主线程发送完成,显式释放其持有的发送端
    // 所有发送端释放且缓冲区为空后,接收迭代才会结束
    // 若保留tx1,主线程等待接收线程退出,接收线程等待tx1释放,形成循环等待
    drop(tx1);
    sender.join().unwrap();
    receiver.join().unwrap();
}
