// Macro std::assert/Macro std::assert_eq/Macro std::assert_ne

#[test]
fn arrert_test() {
    // Asserts that a boolean expression is true at runtime.
    assert!(true);

    // # Custom Messages
    /*
    This macro has a second form, where a custom panic message can be provided with or without arguments for formatting. See std::fmt for syntax for this form. Expressions used as format arguments will only be evaluated if the assertion fails.
     */
    assert!(false, "<<不为true>>");

    let a = 3;
    let b = 27;
    assert!(a + b == 30, "a = {}, b = {}", a, b);
}

#[test]
fn assert_eq_test() {
    let a = 3;
    let b = 1 + 2;
    let c = 4;
    // Asserts that two expressions are equal to each other (using PartialEq).
    assert_eq!(a, b);

    // Like assert!, this macro has a second form, where a custom panic message can be provided.
    assert_eq!(a, b, "<<测试加法运算,测试{}和{}>>", a, b);

    assert_eq!(a, c, "<<测试加法运算,测试{}和{}>>", a, c);
}

#[test]
fn assert_ne_test() {
    let a = 3;
    let b = 2;
    // Asserts that two expressions are not equal to each other (using PartialEq).
    assert_ne!(a, b);

    // Like assert!, this macro has a second form, where a custom panic message can be provided.
    assert_ne!(a, b, "<<测试两个值{}和{}是否不相等>>", a, b);

    assert_ne!(a, a, "<<测试两个值{}和{}是否不相等>>", a, a);
}
