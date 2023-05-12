/*
In Rust, integration tests are entirely external to your library. 
They use your library in the same way any other code would, which means they can only call functions that are part of your library’s public API. 
Their purpose is to test whether many parts of your library work together correctly. 
Units of code that work correctly on their own could have problems when integrated, so test coverage of the integrated code is important as well. 
To create integration tests, you first need a tests directory.
 */

/*
Each file in the tests directory is a separate crate, so we need to bring our library into each test crate’s scope.
For that reason we add use `y_test` at the top of the code, which we didn’t need in the unit tests.
 */
use y_test;

/*
We don’t need to annotate any code in tests/integration_test.rs with #[cfg(test)].
Cargo treats the tests directory specially and compiles files in this directory only when we run cargo test.
 */
#[test]
fn it_adds_two() {
    assert_eq!(4, y_test::add_two(2));
}

#[test]
fn it_adds_two1() {
    assert_eq!(5, y_test::add_two(3));
}