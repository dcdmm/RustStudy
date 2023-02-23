// 字符串切片(Primitive Type str)


// A value of type str is represented the same way as [u8], it is a slice of 8-bit unsigned bytes. 
// However, the Rust standard library makes extra assumptions about str: methods working on str assume and ensure that the data in there is valid UTF-8. 
// Calling a str method with a non-UTF-8 buffer can cause <Undefined Behavior, https://doc.rust-lang.org/reference/behavior-considered-undefined.html> now or in the future.

// Since str is a <dynamically sized type, https://doc.rust-lang.org/reference/dynamically-sized-types.html>, it can only be instantiated through a pointer type, such as &str.
// A &str is made up of two components: a pointer to some bytes, and a length. 
fn main() {
    // String literals are string slices
    let _hello_world = "hello world!";

    let s: String = String::from("hello world");

    let hello: &str = &s[0..5];
    let world: &str = &s[6..11];
    println!("{hello}");
    println!("{world}")
}
