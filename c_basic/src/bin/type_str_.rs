// 字符串切片(Primitive Type str)

// It is usually seen in its borrowed form, &str.
fn main() {
    // String literals are string slices
    let _hello_world = "hello world!";

    let s: String = String::from("hello world");

    let hello: &str = &s[0..5];
    let world: &str = &s[6..11];
    println!("{hello}");
    println!("{world}")
}
