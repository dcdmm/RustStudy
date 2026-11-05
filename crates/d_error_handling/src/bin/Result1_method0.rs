#![allow(non_snake_case)]

// std::result::Result

/*
常见方法:
is_ok / is_err
 */

#[test]
fn is_ok_is_err_test() {
    let x: Result<i32, &str> = Ok(-3);
    // Returns true if the result is Ok.
    println!("{}", x.is_ok());
    // Returns true if the result is Err.
    println!("{}", x.is_err());

    let x: Result<i32, &str> = Err("Some error message");
    println!("{}", x.is_ok());
    println!("{}", x.is_err());
}

fn main() {
}
