// Buffering behavior:
#[test]
fn t034() {
    use std::sync::mpsc;
    use std::sync::mpsc::Receiver;
    use std::sync::mpsc::RecvError;
    use std::sync::mpsc::Sender;
    use std::thread;

    let (send, recv): (Sender<u8>, Receiver<u8>) = mpsc::channel();
    // let handle = thread::spawn(move || {
    //     send.send(1u8).unwrap();
    //     send.send(2).unwrap();
    //     send.send(3).unwrap();
    //     drop(send);
    // });

    // // wait for the thread to join so we ensure the sender is dropped
    // handle.join().unwrap();

    // assert_eq!(Ok(1), recv.recv());
    // assert_eq!(Ok(2), recv.recv());
    // assert_eq!(Ok(3), recv.recv());
    // assert_eq!(Err(RecvError), recv.recv());
}

fn main() {}
