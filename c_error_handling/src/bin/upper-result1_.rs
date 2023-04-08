// Shortcuts for Panic on Error: unwrap and expect

use std::fs::File;

fn main() {
    // panic:thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 2, kind: NotFound, message: "系统找不到指定的文件。" }'
    /*
    Returns the contained Ok value, consuming the self value.

    Because this function may panic, its use is generally discouraged.
    Instead, prefer to use pattern matching and handle the Err case explicitly, or call unwrap_or, unwrap_or_else, or unwrap_or_default.

    # Panics
    Panics if the value is an Err, with a panic message provided by the Err’s value.
     */
    // let greeting_file = File::open("hello.txt").unwrap();

    // panic:thread 'main' panicked at 'hello.txt should be included in this project: Os { code: 2, kind: NotFound, message: "系统找不到指定的文件。" }'
    /*
    Returns the contained Ok value, consuming the self value.

    Because this function may panic, its use is generally discouraged.
    Instead, prefer to use pattern matching and handle the Err case explicitly, or call unwrap_or, unwrap_or_else, or unwrap_or_default.

    # Panics
    Panics if the value is an Err, with a panic message including the passed message, and the content of the Err.
     */
    #[allow(unused)]
    let greeting_file = File::open("hello.txt").expect("hello.txt should be included in this project");
}