#![allow(non_snake_case)]

// Recoverable Errors with Result

/*
Result<T, E> is the type used for returning and propagating errors. It is an enum with the variants, Ok(T), representing success and containing a value, and Err(E), representing error and containing an error value.
```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```
 */
#[allow(warnings)]
use std::fs::File;
#[allow(warnings)]
use std::io::ErrorKind;

#[allow(warnings)]
#[test]
fn t0() {
    let greeting_file_result = File::open("hello_base0.txt");
    let greeting_file = match greeting_file_result {
        // When the result is Ok, this code will return the inner file value out of the Ok variant, and we then assign that file handle value to the variable greeting_file. After the match, we can use the file handle for reading or writing.
        Ok(file) => file,
        // The other arm of the match handles the case where we get an Err value from File::open. In this example, we’ve chosen to call the panic! macro.
        Err(error) => panic!("Problem opening the file: {:?}", error),
    };
}

// Handling different kinds of errors in different ways
#[allow(warnings)]
#[test]
fn t1() {
    let greeting_file_result = File::open("hello_base1.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello_base1.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };
}

fn main() {}
