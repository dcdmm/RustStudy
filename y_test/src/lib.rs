#![allow(warnings)]

mod base0_;
mod base1_macro_assert_;
mod base2_attribute_should_panic_;
mod base3_result_Result_;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_private_functions() {
        // Testing Private Functions
        assert_eq!(4, internal_adder(2, 2));
    }
}
