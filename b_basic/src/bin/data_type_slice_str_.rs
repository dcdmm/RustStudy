// 字符串切片/string slice(Primitive Type str)

fn main() {
    // String literals are string slices:
    let _hello_world: &str = "hello world!";

    // Here we have declared a string slice initialized with a string literal. String literals have a static lifetime, which means the string hello_world is guaranteed to be valid for the duration of the entire program. We can explicitly specify hello_world’s lifetime as well:
    let _hello_rust: &'static str = "hello_rust";

    // String转&str
    let s: String = String::from("hello world 中国");
    let s_str_all0: &str = &s;
    let s_str_all1 = &s[..];
    let s_str_all2 = s.as_str();
    let hello: &str = &s[0..5];
    println!("{s_str_all0}");
    println!("{s_str_all1}");
    println!("{s_str_all2}");
    println!("{hello}");

    let china_get = s.get(14..16);
    match china_get {
        Some(c) => println!("{}", c),
        None => println!("None")
    }
    // panic:byte index 14 is not a char boundary; it is inside '中' (bytes 12..15) of `hello world 中国`
    let _china_str = &s[14..15];
}
