use x_tests;

#[test]
fn t0() {
    assert_eq!(4, x_tests::add_two(2));
}

#[test]
fn t01() {
    assert_eq!(5, x_tests::add_two(3));
}
