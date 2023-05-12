mod ordinary;

/*
The three sections of output include the unit tests, the integration test, and the doc tests.
Note that if any test in a section fails, the following sections will not be run.
For example, if a unit test fails, there won’t be any output for integration and doc tests because those tests will only be run if all unit tests are passing.
 */

pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[allow(dead_code)]
fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

#[derive(Debug)]
#[allow(dead_code)]
struct Rectangle {
    width: u32,
    height: u32,
}

#[allow(dead_code)]
impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

/*
The #[cfg(test)] annotation on the tests module tells Rust to compile and run the test code only when you run cargo test, not when you run cargo build. 
This saves compile time when you only want to build the library and saves space in the resulting compiled artifact because the tests are not included.
 */
#[cfg(test)]
mod tests {
    // use crate::*;
    use super::*; // 与上等价

    /*
    Note the #[test] annotation: this attribute indicates this is a test function, so the test runner knows to treat this function as a test.
    We might also have non-test functions in the tests module to help set up common scenarios or perform common operations, so we always need to indicate which functions are tests.
     */
    #[test]
    fn test_function_panics() {
        /*
        Tests fail when something in the test function panics.
        Each test is run in a new thread, and when the main thread sees that a test thread has died, the test is marked as failed.
        */
        panic!("Make this test fail");
    }

    #[test]
    fn macro_std_assert0() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };

        /*
        Asserts that a boolean expression is true at runtime.

        This will invoke the panic! macro if the provided expression cannot be evaluated to true at runtime.
         */
        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn macro_std_assert1() {
        // assert with a custom message
        let a = 3;
        let b = 27;
        /*
        Custom Messages
        This macro has a second form, where a custom panic message can be provided with or without arguments for formatting.
        See std::fmt for syntax for this form.
        Expressions used as format arguments will only be evaluated if the assertion fails.
         */
        assert!(a + b == 31, "a 等于: {}, b 等于: {}", a, b);
    }

    #[test]
    fn macro_std_assert_eq0() {
        /*
        Asserts that two expressions are equal to each other (using PartialEq).

        On panic, this macro will print the values of the expressions with their debug representations.

        Like assert!, this macro has a second form, where a custom panic message can be provided.
         */
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn macro_std_assert_eq1() {
        let a = 3;
        let b = 1 + 5;

        assert_eq!(a, b, "we are testing addition with {} and {}", a, b);
    }

    #[test]
    fn macro_std_assert_ne() {
        /*
        Asserts that two expressions are not equal to each other (using PartialEq).

        On panic, this macro will print the values of the expressions with their debug representations.

        Like assert!, this macro has a second form, where a custom panic message can be provided.
         */
        assert_ne!(4, add_two(2));
    }

    // The should_panic attribute makes the test only pass if it actually panics.
    #[test]
    // #[should_panic]
    // To make should_panic tests more precise, we can add an optional expected parameter to the should_panic attribute. The test harness will make sure that the failure message contains the provided text.
    #[should_panic(expected = "less than or equal to 100")]
    fn attribute_should_panic() {
        pub struct Guess {
            #[allow(dead_code)]
            value: i32,
        }

        impl Guess {
            pub fn new(value: i32) -> Guess {
                if value < 1 {
                    panic!(
                        "Guess value must be greater than or equal to 1, got {}.",
                        value
                    );
                } else if value > 100 {
                    panic!(
                        "Guess value must be less than or equal to 100, got {}.",
                        value
                    );
                }

                Guess { value }
            }
        }

        Guess::new(200);
    }

    /*
    Writing tests so they return a Result<T, E> enables you to use the question mark operator in the body of tests, which can be a convenient way to write tests that should fail if any operation within them returns an Err variant.

    You can’t use the #[should_panic] annotation on tests that use Result<T, E>.
    To assert that an operation returns an Err variant, don’t use the question mark operator on the Result<T, E> value. Instead, use assert!(value.is_err()).
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
