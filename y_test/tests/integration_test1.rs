use y_test;

/*
To avoid having `common` appear in the test output, instead of creating tests/common.rs(方式一), we’ll create tests/common/mod.rs(方式二). 

After we’ve created tests/common/mod.rs, we can use it from any of the integration test files as a module.
 */

mod common;

#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, y_test::add_two(2));
}