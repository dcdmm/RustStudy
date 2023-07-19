// 字符串切片(Primitive Type str)

/*
The str type, also called a ‘string slice’, is the most primitive string type.
It is usually seen in its borrowed form, &str. It is also the type of string literals, &'static str.

A &str is made up of two components: a pointer to some bytes, and a length.
 */

fn main() {
    let _hello_world = "hello world!";

    let s: String = String::from("hello world");

    let hello: &str = &s[0..5];
    let world = &s[6..11];
    println!("{hello}");
    println!("{world}")
}
