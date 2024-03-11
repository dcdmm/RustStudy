// Shortcuts for Panic on Error: unwrap and expect

#[allow(warnings)]
use std::fs::File;

#[allow(warnings)]
#[test]
fn unwarp_test0() {
    /*
    Returns the contained Ok value, consuming the self value.

    # Panics
    Panics if the value is an Err, with a panic message provided by the Err’s value.
     */
    let x: Result<u32, &str> = Ok(2);
    println!("{}", x.unwrap()); // print->2
}

#[allow(warnings)]
#[test]
fn unwarp_test1() {
    let y: Result<u32, &str> = Err("emergency failure");
    // panicked:called `Result::unwrap()` on an `Err` value: "emergency failure"
    println!("{}", y.unwrap());
}

#[allow(warnings)]
#[test]
fn unwarp_test2() {
    // panicked:called `Result::unwrap()` on an `Err` value: Os { code: 2, kind: NotFound, message: "系统找不到指定的文件。" }
    let greeting_file = File::open("hello_unwrap.txt").unwrap();
}

#[allow(warnings)]
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

#[allow(warnings)]
fn main() {
}
