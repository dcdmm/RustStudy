pub fn add(left: usize, right: usize) -> usize {
    left + right
}

// #[cfg(test)]: 条件编译,被修饰的item仅在测试构建(如cargo test)时编译,普通构建(如cargo build)时不编译
#[cfg(test)]
mod tests {
    use super::*;

    // The test attribute marks a function to be executed as a test. These functions are only compiled when in test mode.
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
