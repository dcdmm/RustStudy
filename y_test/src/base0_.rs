pub fn add(left: usize, right: usize) -> usize {
    left + right
}

/*
The #[cfg(test)] annotation on the tests module tells Rust to compile and run the test code only when you run cargo test, not when you run cargo build. This saves compile time when you only want to build the library and saves space in the resulting compiled artifact because the tests are not included. You’ll see that because integration tests go in a different directory, they don’t need the #[cfg(test)] annotation. However, because unit tests go in the same files as the code, you’ll use #[cfg(test)] to specify that they shouldn’t be included in the compiled result.
 */
#[cfg(test)]
mod tests {
    use super::*;

    // 测试函数(fn前添加#[test])
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
