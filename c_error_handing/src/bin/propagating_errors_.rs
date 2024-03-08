// Propagating Errors

use std::error::Error;
use std::fs::File;
use std::io::{self, Read};

// A function that returns errors to the calling code using match
// Listing 1
#[allow(unused)]
fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

// 与上等价(更加简洁)
// A function that returns errors to the calling code using the ? operator
#[allow(unused)]
fn read_username_from_file_simple() -> Result<String, io::Error> {
    // let mut username_file = match File::open("hello.txt") {
    //     Ok(file) => file,
    //     Err(e) => return Err(e),
    // };
    // 与上等价
    /*
    There is a difference between what the match expression from Listing 1 does and what the ? operator does:
    error values that have the ? operator called on them go through the from function, defined in the From trait in the standard library, which is used to convert values from one type into another.
    When the ? operator calls the from function, the error type received is converted into the error type defined in the return type of the current function.
    This is useful when a function returns one error type to represent all the ways a function might fail, even if parts might fail for many different reasons.
     */
    let mut username_file = File::open("hello.txt")?;

    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

// 与上等价(更加简洁)
// Chaining method calls after the ? operator
#[allow(unused)]
fn read_username_from_file_very_simple() -> Result<String, io::Error> {
    let mut username = String::new();

    File::open("hello.txt")?.read_to_string(&mut username)?;

    Ok(username)
}

// Using the ? operator on an Option<T> value
#[allow(unused)]
fn last_char_of_first_line(text: &str) -> Option<char> {
    /*
    The error message also mentioned that ? can be used with Option<T> values as well.
    As with using ? on Result, you can only use ? on Option in a function that returns an Option.
    The behavior of the ? operator when called on an Option<T> is similar to its behavior when called on a Result<T, E>: if the value is None, the None will be returned early from the function at that point.
    If the value is Some, the value inside the Some is the resulting value of the expression and the function continues.
     */
    text.lines().next()?.chars().last()
}

// #[test]
// fn t00() {
//     // 报错:the `?` operator can only be used in a function that returns `Result` or `Option` (or another type that implements `FromResidual`)
//     let greeting_file = File::open("hello.txt")?;
// }

fn main() {
    fn t0_inner() -> Result<(), Box<dyn Error>> {
        let greeting_file = File::open("hello.txt")?;
        Ok(())
    }

    let result = t0_inner();
    println!("{:?}", result);
}
