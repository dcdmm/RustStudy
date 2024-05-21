// 参考: https://doc.rust-lang.org/reference/attributes/diagnostics.html
// 参考: https://doc.rust-lang.org/rustc/lints/index.html

// The unstable_name_collisions lint detects that you have used a name that the standard library plans to add in the future.
#![allow(unstable_name_collisions)] // 默认为warn
#![warn(unstable_name_collisions)]
#![deny(unstable_name_collisions)]
#![forbid(unstable_name_collisions)]

#[test]
fn lint_unstable_name_collisions_test() {
    trait MyIterator: Iterator {
        // is_sorted is an unstable method that already exists on the Iterator trait
        fn is_sorted(self) -> bool
        where
            Self: Sized,
        {
            true
        }
    }

    impl<T: ?Sized> MyIterator for T where T: Iterator {}

    let x = vec![1, 2, 3];

    // 默认时警告信息===>warning: a method with this name may be added to the standard library in the future 

    // error: a method with this name may be added to the standard library in the future
    // let _ = x.iter().is_sorted();
}

// 默认为warn
// The warnings lint allows you to change the level of other lints which produce warnings.
#[allow(warnings)]
#[test]
fn lint_warngings_test() {
    // 默认时警告信息===>warning: unused variable: `a`
    let a = 4.34;
}

// 默认为warn
// The non_snake_case lint detects variables, methods, functions, lifetime parameters and modules that don't have snake case names.
#[allow(non_snake_case)]
#[test]
fn link_non_snake_case_test() {
    // 默认时警告信息===>warning: variable `MY_VALUE` should have a snake case name
    let MY_VALUE = 5;
}

// 默认为warn
// The dead_code lint detects unused, unexported items.
#[allow(dead_code)]
#[test]
fn link_dead_code_test() {
    // 默认时警告信息===>warning: function `foo` is never used
    fn foo() {}
}
