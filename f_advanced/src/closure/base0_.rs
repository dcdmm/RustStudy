// 闭包

/*
Rust’s closures are anonymous functions you can save in a variable or pass as arguments to other functions. You can create the closure in one place and then call the closure elsewhere to evaluate it in a different context. Unlike functions, closures can capture values from the scope in which they’re defined.
*/

#[allow(warnings)]
#[test]
fn t0() {
    fn add_v1(x: i32) -> i32 {
        x + 1
    } // 函数定义

    let add_v2 = |x: i32| -> i32 { x + 1 }; // fully annotated closure definition
    let v2_s0 = add_v2(5);
    // error[E0308]: mismatched types
    // let v2_s1 = add_v2(5.1);
}

#[allow(warnings)]
#[test]
fn t1() {
    let add_v3 = |x| x + 1; // 省略参数和返回值类型;闭包体只有1行代码(语句或表达式)且未声明返回值类型时,可以省略花括号
    let v3_s0 = add_v3(4); // 推断闭包add_v3 x的类型为i32,返回值的类型为i32

    // error[E0308]: mismatched types
    // let v3_s1 = add_v3(4.1); // 确定(声明/推断)闭包类型(参数/返回值)后,使用时类型不匹配将报错
}

#[test]
fn t2() {
    // error[E0284]: type annotations needed
    // let no_determine = |x| -> &str {"hello rust!"};  // 存在未明确的类型(参数/返回值)时,必须声明Type annotations(如测试t0 闭包add_v2)或编译器可以进行推断(见测试t1 闭包add_v3)
}

#[test]
fn t3() {
    // let x0 = 4;
    // fn equal_to_x0(z: i32) -> bool {
    //     // error[E0434]: can't capture dynamic environment in a fn item
    //     z == x0
    // }
    // let _is_equal_func = equal_to_x0(4);

    let x1 = 4;
    let equal_to_x1 = |z| z == x1; // 捕获作用域中的值
    let _is_equal_closure = equal_to_x1(4);
}
