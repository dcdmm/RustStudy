// 参考: https://doc.rust-lang.org/reference/attributes/diagnostics.html
// 参考: https://doc.rust-lang.org/rustc/lints/index.html

// 每个.rs文件都是一个模块,在文件开头添加#![attribute],作用于该文件对应的模块

// lint级别:
// #![allow(unused_mut)] // 允许
// #![warn(unused_mut)] // 警告
#![deny(unused_mut)] // 产生error,内层可用#[allow]覆盖
// #![forbid(unused_mut)] // 产生error,内层不可覆盖

// 默认为warn
// The unused_mut lint detects mut variables which don't need to be mutable.
#[test]
// #[allow(unused_mut)]  // 取消注释后覆盖外层deny,该函数内不再产生error
fn lint_unused_mut_test() {
    // 默认时警告信息===>warning: variable does not need to be mutable

    // #[allow(unused_mut)]  // (最内层)取消注释后覆盖外层deny,该语句不再产生error
    // error: variable does not need to be mutable
    // let mut x = 5;
    // println!("{x}");
}

// 默认为warn
// The warnings lint allows you to change the level of other lints which produce warnings.
#[allow(warnings)]
#[test]
fn lint_warngings_test() {
    // 默认时警告信息===>warning: unused variable: `a`
    let a = 4.34;
}

// non_snake_case: 默认为warn
// The non_snake_case lint detects variables, methods, functions, lifetime parameters and modules that don't have snake case names.
//
// unused_variables: 默认为warn
// The unused_variables lint detects variables which are not used in any way.
#[allow(non_snake_case, unused_variables)]
#[test]
fn lint_non_snake_unused_variables_case_test() {
    // 默认时警告信息===>warning: variable `MY_VALUE` should have a snake case name
    // 默认时警告信息===>warning: unused variable: `MY_VALUE`
    let MY_VALUE = 5;
}

// 默认为warn
// The dead_code lint detects unused, unexported items.
#[allow(dead_code)]
#[test]
fn lint_dead_code_test() {
    // 默认时警告信息===>warning: function `foo` is never used
    fn foo() {}
}
