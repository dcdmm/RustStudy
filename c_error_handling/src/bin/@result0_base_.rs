// Recoverable Errors with Result

/*
Error handling with the Result type.

Result<T, E> is the type used for returning and propagating errors.
It is an enum with the variants, Ok(T), representing success and containing a value, and Err(E), representing error and containing an error value.

enum Result<T, E> {
   Ok(T),
   Err(E),
}
 */
use std::fs::File;
use std::io::ErrorKind;

#[allow(unused)]
fn main() {
    // let greeting_file_result: Result<T, E> = File::open("hello.txt");
    //
    // // panic:thread 'main' panicked at 'Problem opening the file: Os { code: 2, kind: NotFound, message: "系统找不到指定的文件。" }'
    // let greeting_file = match greeting_file_result {
    //     /*
    //     When the result is Ok, this code will return the inner file value out of the Ok variant, and we then assign that file handle value to the variable greeting_file.
    //     After the match, we can use the file handle for reading or writing.
    //      */
    //     Ok(file) => file,
    //     // The other arm of the match handles the case where we get an Err value from File::open. In this example, we’ve chosen to call the panic! macro.
    //     Err(error) => panic!("Problem opening the file: {:?}", error),
    // };

    // *****************************************************************************
    let greeting_file_result = File::open("hello.txt");

    // Matching on Different Errors
    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) =>
            match error.kind() {
                ErrorKind::NotFound =>
                    match File::create("hello.txt") {
                        Ok(fc) => fc,
                        Err(e) => panic!("Problem creating the file: {:?}", e),
                    }
                other_error => {
                    panic!("Problem opening the file: {:?}", other_error);
                }
            },
    };
}