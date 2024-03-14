// 闭包

#[allow(warnings)]
#[test]
fn t0() {
    fn add_v1(x: i32) -> i32 {
        x + 1
    } //  function definition

    let add_v2 = |x: i32| -> i32 { x + 1 }; // fully annotated closure definition
    let v2_s0 = add_v2(5);
    // error[E0308]: mismatched types
    // let v2_s1 = add_v2(5.1);
}

#[allow(warnings)]
#[test]
fn t1() {
    let add_v3 = |x| x + 1; // 闭包体只有1行代码(语句或表达式)且未声明返回值类型时,可以省略花括号
    let v3_s0 = add_v3(4); // 推断闭包add_v3 x的类型为i32,返回值的类型为i32

    // error[E0308]: mismatched types
    // let v3_s1 = add_v3(4.1); // `4.1`: expected integer, found floating-point number
}

#[test]
fn t2() {
    // error[E0284]: type annotations needed
    // let add_v4 = |x| x + 1;  // 必须声明Type annotations(见测试t0闭包add_v2)或编译器可以进行推断(见测试t1闭包add_v3)
}
