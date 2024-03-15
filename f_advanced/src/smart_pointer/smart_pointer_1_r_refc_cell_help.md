Similar to Rc<T>, RefCell<T> is only for use in single-threaded scenarios and will give you a compile-time error if you try using it in a multithreaded context. 

Here is a recap of the reasons to choose Box<T>, Rc<T>, or RefCell<T>:

* Rc<T> enables multiple owners of the same data; Box<T> and RefCell<T> have single owners.
* Box<T> allows immutable or mutable borrows checked at compile time; Rc<T> allows only immutable borrows checked at compile time; RefCell<T> allows immutable or mutable borrows checked at runtime.
* Because RefCell<T> allows mutable borrows checked at runtime, you can mutate the value inside the RefCell<T> even when the RefCell<T> is immutable.

```rust
pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
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

#[cfg(test)]
mod tests {
    use super::*;

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
        fn send(&self, message: &str) {
            self.sent_messages.push(String::from(message));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);

        assert_eq!(mock_messenger.sent_messages.len(), 1);
    }
}

fn main() {

}

/*
程序运行结果(报错):
 error[E0596]: cannot borrow `self.sent_messages` as mutable, as it is behind a `&` reference
  --> d_advanced\src\main.rs:58:13
   |
58 |             self.sent_messages.push(String::from(message));
   |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ `self` is a `&` reference, so the data it refers to cannot be borrowed as mutable
   |
help: consider changing that to be a mutable reference
   |
2  |     fn send(&mut self, msg: &str);
   |             ~~~~~~~~~
 */
```

We can’t modify the MockMessenger to keep track of the messages, because the send method takes an immutable reference to self. We also can’t take the suggestion from the error text to use &mut self instead, because then the signature of send wouldn’t match the signature in the Messenger trait definition.

```rust
// Listing 1: Using RefCell<T> to mutate an inner value while the outer value is considered immutable

pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
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

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use super::*;

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
            /*
            For the implementation of the send method, the first parameter is still an immutable borrow of self, which matches the trait definition.
            We call borrow_mut on the RefCell<Vec<String>> in self.sent_messages to get a mutable reference to the value inside the RefCell<Vec<String>>, which is the vector.
            Then we can call push on the mutable reference to the vector to keep track of the messages sent during the test.
             */
            self.sent_messages.borrow_mut().push(String::from(message));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);

        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }
}

fn main() {}
```

When creating immutable and mutable references, we use the & and &mut syntax, respectively. With RefCell<T>, we use the borrow and borrow_mut methods, which are part of the safe API that belongs to RefCell<T>. The borrow method returns the smart pointer type Ref<T>, and borrow_mut returns the smart pointer type RefMut<T>. Both types implement Deref, so we can treat them like regular references.

The RefCell<T> keeps track of how many Ref<T> and RefMut<T> smart pointers are currently active. Every time we call borrow, the RefCell<T> increases its count of how many immutable borrows are active. When a Ref<T> value goes out of scope, the count of immutable borrows goes down by one. Just like the compile-time borrowing rules, RefCell<T> lets us have many immutable borrows or one mutable borrow at any point in time.

If we try to violate these rules, rather than getting a compiler error as we would with references, the implementation of RefCell<T> will panic at runtime. Listing 2 shows a modification of the implementation of send in Listing 1. We’re deliberately trying to create two mutable borrows active for the same scope to illustrate that RefCell<T> prevents us from doing this at runtime.

```rust
// Listing 2: Creating two mutable references in the same scope to see that RefCell<T> will panic

pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
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

#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use super::*;

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
            /*
            We create a variable one_borrow for the RefMut<T> smart pointer returned from borrow_mut. 
            Then we create another mutable borrow in the same way in the variable two_borrow. 
            This makes two mutable references in the same scope, which isn’t allowed.
             When we run the tests for our library, the code in Listing 2 will compile without any errors, but the test will fail:
             */
            let mut one_borrow = self.sent_messages.borrow_mut();
            let mut two_borrow = self.sent_messages.borrow_mut();

            one_borrow.push(String::from(message));
            two_borrow.push(String::from(message));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock_messenger = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

        limit_tracker.set_value(80);

        assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
    }
}

fn main() {}

/*
程序运行结果(panic):
already borrowed: BorrowMutError
thread 'tests::it_sends_an_over_75_percent_warning_message' panicked at 'already borrowed: BorrowMutError', d_advanced\src\bin\smart_pointer_r_refc_cell0_:62:53
stack backtrace:
   0: std::panicking::begin_panic_handler
             at /rustc/9eb3afe9ebe9c7d2b84b71002d44f4a0edac95e0/library\std\src\panicking.rs:575
   1: core::panicking::panic_fmt
             at /rustc/9eb3afe9ebe9c7d2b84b71002d44f4a0edac95e0/library\core\src\panicking.rs:64
   2: core::result::unwrap_failed
             at /rustc/9eb3afe9ebe9c7d2b84b71002d44f4a0edac95e0/library\core\src\result.rs:1790
   3: enum2$<core::result::Result<core::cell::RefMut<alloc::vec::Vec<alloc::string::String,alloc::alloc::Global> >,core::cell::BorrowMutError> >::expect<core::cell::RefMut<alloc::vec::Vec<alloc::string::String,alloc::alloc::Global> >,core::cell::BorrowMutError>
             at /rustc/9eb3afe9ebe9c7d2b84b71002d44f4a0edac95e0\library\core\src\result.rs:1069
   4: core::cell::RefCell<alloc::vec::Vec<alloc::string::String,alloc::alloc::Global> >::borrow_mut<alloc::vec::Vec<alloc::string::String,alloc::alloc::Global> >
             at /rustc/9eb3afe9ebe9c7d2b84b71002d44f4a0edac95e0\library\core\src\cell.rs:959
   5: smart_pointer_r_refcell0_::tests::impl$1::send
             at .\src\bin\smart_pointer_r_refc_cell0_:62
   6: smart_pointer_r_refcell0_::LimitTracker<smart_pointer_r_refcell0_::tests::MockMessenger>::set_value<smart_pointer_r_refcell0_::tests::MockMessenger>
             at .\src\bin\smart_pointer_r_refc_cell0_:36
   7: smart_pointer_r_refcell0_::tests::it_sends_an_over_75_percent_warning_message
             at .\src\bin\smart_pointer_r_refc_cell0_:74
   8: smart_pointer_r_refcell0_::tests::it_sends_an_over_75_percent_warning_message::closure$0
             at .\src\bin\smart_pointer_r_refc_cell0_:70
   9: core::ops::function::FnOnce::call_once<smart_pointer_r_refcell0_::tests::it_sends_an_over_75_percent_warning_message::closure_env$0,tuple$<> >
             at /rustc/9eb3afe9ebe9c7d2b84b71002d44f4a0edac95e0\library\core\src\ops\function.rs:250
  10: core::ops::function::FnOnce::call_once
             at /rustc/9eb3afe9ebe9c7d2b84b71002d44f4a0edac95e0/library\core\src\ops\function.rs:250
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

error: test failed, to rerun pass `--bin smart_pointer_r_refcell0_`
error: 1 target failed:
    `--bin smart_pointer_r_refcell0_`
 */
```

Notice that the code panicked with the message already borrowed: BorrowMutError. This is how RefCell<T> handles violations of the borrowing rules at runtime.

Choosing to catch borrowing errors at runtime rather than compile time, as we’ve done here, means you’d potentially be finding mistakes in your code later in the development process: possibly not until your code was deployed to production. Also, your code would incur a small runtime performance penalty as a result of keeping track of the borrows at runtime rather than compile time. However, using RefCell<T> makes it possible to write a mock object that can modify itself to keep track of the messages it has seen while you’re using it in a context where only immutable values are allowed. You can use RefCell<T> despite its trade-offs to get more functionality than regular references provide.