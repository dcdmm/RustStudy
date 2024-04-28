/*
There are two styles of tests within a Cargo project:

* Unit tests which are functions marked with the #[test] attribute located within your library or binaries (or any target enabled with the test field). These tests have access to private APIs located within the target they are defined in.

* Integration tests* which is a separate executable binary, also containing `#[test]` functions, which is linked with the project’s library and has access to its *public* API.
 */

/*
We create a tests directory at the top level of our project directory, next to src. Cargo knows to look for integration test files in this directory. We can then make as many test files as we want, and Cargo will compile each of the files as an individual crate.

Each file in the tests directory is a separate crate, so we need to bring our library into each test crate’s scope. For that reason we add use `y_test` at the top of the code, which we didn’t need in the unit tests.

We don’t need to annotate any code in tests/integration_test.rs with #[cfg(test)]. Cargo treats the tests directory specially and compiles files in this directory only when we run cargo test.

The three sections of output include the unit tests, the integration test, and the doc tests. Note that if any test in a section fails, the following sections will not be run. For example, if a unit test fails, there won’t be any output for integration and doc tests because those tests will only be run if all unit tests are passing.
*/

use y_test;

#[test]
fn t0() {
    assert_eq!(4, y_test::add_two(2));
}

#[test]
fn t1() {
    assert_eq!(5, y_test::add_two(3));
}
