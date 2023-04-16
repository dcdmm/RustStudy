// 闭包

/*
There are more differences between functions and closures.
Closures don’t usually require you to annotate the types of the parameters or the return value like fn functions do.
Type annotations are required on functions because the types are part of an explicit interface exposed to your users.
Defining this interface rigidly is important for ensuring that everyone agrees on what types of values a function uses and returns.
Closures, on the other hand, aren’t used in an exposed interface like this: they’re stored in variables and used without naming them and exposing them to users of our library.

Closures are typically short and relevant only within a narrow context rather than in any arbitrary scenario.
Within these limited contexts, the compiler can infer the types of the parameters and the return type, similar to how it’s able to infer the types of most variables (there are rare cases where the compiler needs closure type annotations too).

As with variables, we can add type annotations if we want to increase explicitness and clarity at the cost of being more verbose than is strictly necessary.
*/

#[allow(dead_code)]
fn main() {
    fn add_one_v1(x: u32) -> u32 { x + 1 } //  function definition

    let add_one_v2 = |x: u32| -> u32 { x }; //  fully annotated closure definition
    let _v2_s0 = add_one_v2(5);
    // 报错:mismatched types
    // let v2_s1 = add_one_v2(5.1);

    let add_one_v3 = |x| { x + 1 };
    let _v3_s0 = add_one_v3(4); // For closure definitions, the compiler will infer one concrete type for each of their parameters and for their return value.
    // 报错:mismatched types
    // let v3_s1 = add_one_v3(4.1); // 闭包已经推断出参数类型(i32)和返回值类型(i32)

    // 报错:type annotations needed
    // we remove the brackets, which are optional because the closure body has only one expression.
    // let add_one_v4 = |x| x + 1; // 必须手动添加类型标注,或者编译器可以自动推断
}
