/*
For any lint check C:
    * allow(C) overrides the check for C so that violations will go unreported,
    * warn(C) warns about violations of C but continues compilation.
    * deny(C) signals an error after encountering a violation of C,
*/

// 参考: https://doc.rust-lang.org/rustc/lints/listing/allowed-by-default.html

// The warnings lint allows you to change the level of other lints which produce warnings.
#[allow(warnings)]
#[test]
fn warngings_test() {
    // warning: unused variable: `a`
    let a = 4.34;
}

// The non_snake_case lint detects variables, methods, functions, lifetime parameters and modules that don't have snake case names.
#[allow(non_snake_case)]
#[test]
fn non_snake_case_test() {
    // warning: variable `MY_VALUE` should have a snake case name
    let MY_VALUE = 5;
}

#[allow(dead_code)]
#[test]
fn dead_code_test() {
    // warning: function `foo` is never used
    fn foo() {}
}
