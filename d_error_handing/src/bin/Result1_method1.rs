#![allow(non_snake_case)]
#![allow(warnings)]

// Shortcuts for Panic on Error

/*
常见方法:
ok / err
unwarp 
unwarp_err
expect
 */

use std::fs::File;

#[test]
fn ok_err_test() {
    let x: Result<u32, &str> = Ok(2);
    
    let x_mctch = match x {
        Ok(z) => Some(z),
        Err(_) => None,
    };
    println!("{:?}", x_mctch); // print->Some(2)
    /*
    Converts from Result<T, E> to Option<T>.

    Converts self into an Option<T>, consuming self, and discarding the error, if any.
     */
    println!("{:?}", x.ok()); // 与上等价;print->Some(2)

    /*
    Converts from Result<T, E> to Option<E>.

    Converts self into an Option<E>, consuming self, and discarding the success value, if any.
     */
    println!("{:?}", x.err()); // print->None

    let y: Result<u32, &str> = Err("Nothing here");
    println!("{:?}", y.ok()); // print->None
    println!("{:?}", y.err()); // print->Some("Nothing here")
}

#[test]
fn unwrap_test0() {
    /*
    Returns the contained Ok value, consuming the self value.

    # Panics
    Panics if the value is an Err, with a panic message provided by the Err’s value.
     */
    let x: Result<u32, &str> = Ok(2);
    println!("{}", x.unwrap()); // print->2
}

#[test]
fn unwrap_test1() {
    let y: Result<u32, &str> = Err("emergency failure");
    // panicked:called `Result::unwrap()` on an `Err` value: "emergency failure"
    println!("{}", y.unwrap());
}

#[test]
fn unwrap_test2() {
    // panicked:called `Result::unwrap()` on an `Err` value: Os { code: 2, kind: NotFound, message: "系统找不到指定的文件。" }
    let greeting_file = File::open("hello_unwrap.txt").unwrap();
}

#[test]
fn unwrap_err_test() {
    let x: Result<u32, &str> = Err("emergency failure");
    /*
    Returns the contained Err value, consuming the self value.

    # Panics
    Panics if the value is an Ok, with a custom panic message provided by the Ok’s value.
     */
    assert_eq!(x.unwrap_err(), "emergency failure");

    let x: Result<u32, &str> = Ok(2);
    // panicked:called `Result::unwrap_err()` on an `Ok` value: 2
    x.unwrap_err(); 
}

#[test]
fn expect_test0() {
    /*
    Returns the contained Ok value, consuming the self value.

    # Panics
    Panics if the value is an Err, with a panic message including the passed message, and the content of the Err.
     */
    let x: Result<u32, &str> = Err("emergency failure");
    // panicked:Testing expect: "emergency failure"
    x.expect("Testing expect"); 
}

fn main() {
}
