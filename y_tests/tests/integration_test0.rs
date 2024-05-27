use y_test;

#[test]
fn t0() {
    assert_eq!(4, y_test::add_two(2));
}

#[test]
fn t01() {
    assert_eq!(5, y_test::add_two(3));
}
