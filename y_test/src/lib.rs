#![allow(warnings)]

mod base0_;
mod base1_macro_assert_;



mod ordinary;

/*
The three sections of output include the unit tests, the integration test, and the doc tests.
Note that if any test in a section fails, the following sections will not be run.
For example, if a unit test fails, there won’t be any output for integration and doc tests because those tests will only be run if all unit tests are passing.
 */

pub fn add_two(a: i32) -> i32 {
    a + 2
}

fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}


#[cfg(test)]
mod tests {
    // use crate::*;
    use super::*; // 与上等价

    /*
    Writing tests so they return a Result<T, E> enables you to use the question mark operator in the body of tests, which can be a convenient way to write tests that should fail if any operation within them returns an Err variant.

    You can’t use the #[should_panic] annotation on tests that use Result<T, E>. To assert that an operation returns an Err variant, don’t use the question mark operator on the Result<T, E> value. Instead, use assert!(value.is_err()).
     */
    #[test]
    fn return_r_result() -> Result<(), String> {
        if 2 + 2 == 5 {
            Ok(())
        } else {
            Err(String::from("two plus two does not equal four"))
        }
    }

    #[test]
    fn test_private_functions() {
        // Testing Private Functions
        assert_eq!(4, internal_adder(2, 2));
    }
}
