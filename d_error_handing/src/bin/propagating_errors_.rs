// Propagating Errors

use std::error::Error;
use std::fs::File;
use std::io::Read;

#[allow(warnings)]
fn read_username_from_file() -> Result<String, Box<dyn std::error::Error>> {
    let username_file_result = File::open("hello_propagating_errors0.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(Box::new(e)),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(Box::new(e)),
    }
}

// 与上等价且更加简洁
// A function that returns errors to the calling code using the ? operator
#[allow(warnings)]
fn read_username_from_file_simple() -> Result<String, Box<dyn std::error::Error>> {
    /*
    If the value of the Result is an Ok, the value inside the Ok will get returned from this expression, and the program will continue. If the value is an Err, the Err will be returned from the whole function as if we had used the return keyword so the error value gets propagated to the calling code.

    The ? operator can only be used in functions whose return type is compatible with the value the ? is used on. This is because the ? operator is defined to perform an early return of a value out of the function.
    */
    let mut username_file = File::open("hello_propagating_errors0.txt")?;

    let mut username = String::new();

    username_file.read_to_string(&mut username)?;

    Ok(username)
}

// 与上等价且更加简洁
// Chaining method calls after the ? operator
#[allow(unused)]
fn read_username_from_file_very_simple() -> Result<String, Box<dyn std::error::Error>> {
    let mut username = String::new();

    File::open("hello_propagating_errors0.txt")?.read_to_string(&mut username)?;

    Ok(username)
}

// Using the ? operator on an Option<T> value
#[allow(warnings)]
fn last_char_of_first_line(text: &str) -> Option<char> {
    /*
    The error message also mentioned that ? can be used with Option<T> values as well. As with using ? on Result, you can only use ? on Option in a function that returns an Option. The behavior of the ? operator when called on an Option<T> is similar to its behavior when called on a Result<T, E>: if the value is None, the None will be returned early from the function at that point. If the value is Some, the value inside the Some is the resulting value of the expression and the function continues.
     */
    text.lines().next()?.chars().last()
}

// fn test0_error() {
//     // error[E0277]: the `?` operator can only be used in a function that returns `Result` or `Option` (or another type that implements `FromResidual`)
//     let greeting_file = File::open("hello_propagating_errors0.txt")?;
// }
// 修正如下所示
#[allow(warnings)]
fn test0_right() -> Result<(), Box<dyn Error>> {
    let greeting_file = File::open("hello_propagating_errors0.txt")?;
    Ok(())
}

// fn test_error1(arr: &[i32]) -> Option<&i32> {
//     // error[E0308]: `?` operator has incompatible types
//     // note: `?` operator cannot convert from `&i32` to `Option<&i32>`
//     arr.get(0)?
// }
// 修正如下所示
#[allow(warnings)]
fn test_right1(arr: &[i32]) -> Option<&i32> {
    Some(arr.get(0)?)
}

fn main() {}
