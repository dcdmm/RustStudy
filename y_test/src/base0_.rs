pub fn add(left: usize, right: usize) -> usize {
    left + right
}

// The #[cfg(test)] annotation on the tests module tells Rust to compile and run the test code only when you run cargo test, not when you run cargo build.
#[cfg(test)]
mod tests {
    use super::*;

    /*
    The test attribute

    The test attribute marks a function to be executed as a test. These functions are only compiled when in test mode.
     */
    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn panics_test() {
        /*
        Tests fail when something in the test function panics. Each test is run in a new thread, and when the main thread sees that a test thread has died, the test is marked as failed.
        */
        panic!("Make this test fail");
    }
}
