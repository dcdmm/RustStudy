// std::cell::RefCell

// mutable memory location with dynamically checked borrow rules

// Similar to Rc<T>, RefCell<T> is only for use in single-threaded scenarios and will give you a compile-time error if you try using it in a multithreaded context.

#[test]
fn t0() {
    trait Messenger {
        fn send(&mut self, msg: &str);
    }

    struct LimitTracker<'a, T: Messenger> {
        // 2. 要求messenger必须是可变的
        messenger: &'a mut T,
        value: usize,
        max: usize,
    }

    impl<'a, T> LimitTracker<'a, T>
    where
        // Trait Bound Syntax
        T: Messenger,
    {
        fn new(messenger: &'a mut T, max: usize) -> LimitTracker<'a, T> {
            LimitTracker {
                messenger,
                value: 0,
                max,
            }
        }

        fn set_value(&mut self, value: usize) {
            self.value = value;

            let percentage_of_max = self.value as f64 / self.max as f64;

            if percentage_of_max >= 1.0 {
                self.messenger.send("Error: You are over your quota!");
            } else if percentage_of_max >= 0.9 {
                self.messenger
                    .send("Urgent warning: You've used up over 90% of your quota!");
            } else if percentage_of_max >= 0.75 {
                self.messenger
                    .send("Warning: You've used up over 75% of your quota!");
            }
        }
    }

    struct MockMessenger {
        sent_messages: Vec<String>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: vec![],
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&mut self, message: &str) {
            // 1. 要求send第一个参数必须为可变引用
            self.sent_messages.push(String::from(message));
        }
    }

    let mut mock_messenger = MockMessenger::new();
    let mut limit_tracker = LimitTracker::new(&mut mock_messenger, 100);
    limit_tracker.set_value(80);
    println!("{:?}", limit_tracker.messenger.sent_messages);
    limit_tracker.set_value(20);
    println!("{:?}", limit_tracker.messenger.sent_messages);
}

#[test]
fn t1() {
    use std::cell::RefCell;

    trait Messenger {
        fn send(&self, msg: &str);
    }

    struct LimitTracker<'a, T: Messenger> {
        messenger: &'a T,
        value: usize,
        max: usize,
    }

    impl<'a, T> LimitTracker<'a, T>
    where
        T: Messenger,
    {
        pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
            LimitTracker {
                messenger,
                value: 0,
                max,
            }
        }

        pub fn set_value(&mut self, value: usize) {
            self.value = value;

            let percentage_of_max = self.value as f64 / self.max as f64;

            if percentage_of_max >= 1.0 {
                self.messenger.send("Error: You are over your quota!");
            } else if percentage_of_max >= 0.9 {
                self.messenger
                    .send("Urgent warning: You've used up over 90% of your quota!");
            } else if percentage_of_max >= 0.75 {
                self.messenger
                    .send("Warning: You've used up over 75% of your quota!");
            }
        }
    }

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            // A consequence of the borrowing rules is that when you have an immutable value, you can’t borrow it mutably.
            // Interior Mutability: A Mutable Borrow to an Immutable Value
            self.sent_messages.borrow_mut().push(String::from(message));
        }
    }

    let mut mock_messenger = MockMessenger::new();
    let mut limit_tracker = LimitTracker::new(&mut mock_messenger, 100);
    limit_tracker.set_value(80);
    println!("{:?}", limit_tracker.messenger.sent_messages);
    limit_tracker.set_value(20);
    println!("{:?}", limit_tracker.messenger.sent_messages);
}
