// Macro std::assert/Macro std::assert_eq/Macro std::assert_ne

#[test]
fn arrert_test() {
    // Asserts that a boolean expression is true at runtime.
    assert!(true);

    // # Custom Messages
    /*
    This macro has a second form, where a custom panic message can be provided with or without arguments for formatting. See std::fmt for syntax for this form. Expressions used as format arguments will only be evaluated if the assertion fails.
     */
    // paniced: x wasn't true!
    assert!(false, "x wasn't true!");

    let a = 3;
    let b = 27;
    assert!(a + b == 30, "a = {}, b = {}", a, b);
}

#[test]
fn assert_eq_test() {
    let a = 3;
    let b = 1 + 2;
    // Asserts that two expressions are equal to each other (using PartialEq).
    assert_eq!(a, b);

    // Like assert!, this macro has a second form, where a custom panic message can be provided.
    assert_eq!(a, b, "we are testing addition with {} and {}", a, b);

    /*
    panicked:
    assertion `left == right` failed: we are testing addition with 3 and 3
    left: 3
    right: 4
     */
    assert_eq!(a, 4, "we are testing addition with {} and {}", a, b);
}

#[test]
fn assert_ne_test() {
    let a = 3;
    let b = 2;
    // Asserts that two expressions are not equal to each other (using PartialEq).
    assert_ne!(a, b);

    // Like assert!, this macro has a second form, where a custom panic message can be provided.
    assert_ne!(a, b, "we are testing that the values are not equal");

    /*
    panicked:
    assertion `left != right` failed: we are testing that the values are not equal
    left: 3
    right: 3
     */
    assert_ne!(a, 3, "we are testing that the values are not equal");
}
