// 闭包

// There are more differences between functions and closures. Closures don’t usually require you to annotate the types of the parameters or the return value like fn functions do. Type annotations are required on functions because the types are part of an explicit interface exposed to your users. Defining this interface rigidly is important for ensuring that everyone agrees on what types of values a function uses and returns. Closures, on the other hand, aren’t used in an exposed interface like this: they’re stored in variables and used without naming them and exposing them to users of our library.

// Closures are typically short and relevant only within a narrow context rather than in any arbitrary scenario. Within these limited contexts, the compiler can infer the types of the parameters and the return type, similar to how it’s able to infer the types of most variables (there are rare cases where the compiler needs closure type annotations too).

// As with variables, we can add type annotations if we want to increase explicitness and clarity at the cost of being more verbose than is strictly necessary.

// With type annotations added, the syntax of closures looks more similar to the syntax of functions.
fn main() {
    #[allow(dead_code)]
    fn add_one_v1(x: u32) -> u32 {
        x + 1
    } //  function definition
    let add_one_v2 = |x: u32| -> u32 {x}; //  fully annotated closure definition
    let _v2_s0 = add_one_v2(5);
    // let v2_s1 = add_one_v2(5.1); // 类型不匹配
    // 程序运行结果:
//     error[E0308]: mismatched types
//   --> c_advanced\src\bin\closure0_.rs:17:28
//    |
// 17 |     let v2_s1 = add_one_v2(5.1); // 类型不匹配
//    |                 ---------- ^^^ expected `u32`, found floating-point number
//    |                 |
//    |                 arguments to this function are incorrect
//    |
// note: closure parameter defined here
//   --> c_advanced\src\bin\closure0_.rs:15:23
//    |
// 15 |     let add_one_v2 = |x: u32| -> u32 {x}; //  fully annotated closure definition

    let add_one_v3 = |x| {x + 1};
    let _v3_s0 = add_one_v3(4); // 编译器自动推断出闭包参数类型和返回值类型(唯一)
    // let v3_s1 = add_one_v3(4.1); // 类型不匹配(已经明确闭包参数类型和返回值类型)
    // 程序运行结果:
//     error[E0308]: mismatched types
//   --> c_advanced\src\bin\closure0_.rs:34:28
//    |
// 34 |     let v3_s1 = add_one_v3(4.1); // 类型不匹配(已经明确闭包参数类型和返回值类型)
//    |                 ---------- ^^^ expected integer, found floating-point number
//    |                 |
//    |                 arguments to this function are incorrect
//    |
// note: closure parameter defined here
//   --> c_advanced\src\bin\closure0_.rs:32:23
//    |
// 32 |     let add_one_v3 = |x| {x + 1};
    
    // let add_one_v4 = |x| x + 1; // 要么手动添加类型标注,要么使用闭包(编译器自动推断出闭包参数类型和返回值类型)
    // 程序运行结果:
//     error[E0282]: type annotations needed
//   --> c_advanced\src\bin\closure0_.rs:49:23
//    |
// 49 |     let add_one_v4 = |x| x + 1;
//    |                       ^
//    |
// help: consider giving this closure parameter an explicit type
//    |
// 49 |     let add_one_v4 = |x: _| x + 1;
//    |                        +++
}
