// 生命周期

// We’ve specified a lifetime parameter 'a for the parameter x and the return type, but not for the parameter _y, because the lifetime of _y does not have any relationship with the lifetime of x or the return value.
#[allow(dead_code)]
fn longest<'a>(x: &'a str, _y: &str) -> &'a str {
    x
}

// When returning a reference from a function, the lifetime parameter for the return type needs to match the lifetime parameter for one of the parameters.
// If the reference returned does not refer to one of the parameters, it must refer to a value created within this function.
// However, this would be a dangling reference because the value will go out of scope at the end of the function.
// ======
// #[allow(dead_code)]
// fn longest1<'a>(_x: &str, _y: &str) -> &'a str {
//     let result = String::from("really long string");
//     result.as_str()
// }
// ======
// 程序运行结果:
// error[E0515]: cannot return reference to local variable `result`
//   --> c_advanced\src\bin\lifetime2_.rs:15:5
//    |
// 15 |     result.as_str()
//    |     ^^^^^^^^^^^^^^^ returns a reference to data owned by the current function

// The problem is that result goes out of scope and gets cleaned up at the end of the longest function.
// We’re also trying to return a reference to result from the function.
// There is no way we can specify lifetime parameters that would change the dangling reference, and Rust won’t let us create a dangling reference.
// In this case, the best fix would be to return an owned data type rather than a reference so the calling function is then responsible for cleaning up the value.

fn longest1<'a>(_x: &str, _y: &str) -> String {
    let result = String::from("really long string");
    result
}

fn main() {
    let string1 = String::from("abcd");
    let string2 = "efghijklmnopqrstuvwxyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);

    // ********************************************************
    let string11 = String::from("abcd");
    let string21 = "xyz";

    let result1 = longest1(string11.as_str(), string21);
    println!("The longest string is {}", result1);
}
