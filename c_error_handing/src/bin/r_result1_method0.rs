// std::result::Result

/*
常见方法:
ok / err
is_ok / is_err
 */

#[test]
fn ok_err_test() {
    let x: Result<u32, &str> = Ok(2);
    /*
    Converts from Result<T, E> to Option<T>.

    Converts self into an Option<T>, consuming self, and discarding the error, if any.
     */
    println!("{:?}", x.ok());
    /*
    Converts from Result<T, E> to Option<E>.

    Converts self into an Option<E>, consuming self, and discarding the success value, if any.
     */
    println!("{:?}", x.err());

    let y: Result<u32, &str> = Err("Nothing here");
    println!("{:?}", y.ok());
    println!("{:?}", y.err());
}

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

fn main() {}
