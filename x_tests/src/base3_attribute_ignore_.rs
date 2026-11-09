// Ignoring Some Tests Unless Specifically Requested

// --ignored: Run only ignored tests
// --include-ignored: Run ignored and not ignored tests

// x_tests目录下执行:
// cargo test --bin x_tests base3_attribute_ignore_
// 输出:
// running 2 tests
// test base3_attribute_ignore_::expensive_test ... ignored
// test base3_attribute_ignore_::it_works ... ok
//
// cargo test --bin x_tests base3_attribute_ignore_ -- --ignored
// 输出:
// running 1 test
// test base3_attribute_ignore_::expensive_test ... ok
//
// cargo test --bin x_tests base3_attribute_ignore_ -- --include-ignored
// 输出
// running 2 tests
// test base3_attribute_ignore_::expensive_test ... ok
// test base3_attribute_ignore_::it_works ... ok
#[test]
fn it_works() {
    assert_eq!(2 + 2, 4);
}

#[test]
#[ignore]
fn expensive_test() {
    assert_eq!(2 - 2, 0);
}
