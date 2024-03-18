// 字符串切片/string slice(Primitive Type str)

fn main() {
    // String literals are string slices:
    let _hello_world: &str = "hello world!";

    // String转&str
    let s: String = String::from("hello world 中国");
    let s_str_all0: &str = &s;
    let s_str_all1 = &s[..];
    let s_str_all2 = s.as_str();
    let hello = &s[0..5];
    let hello_get = s.get(0..5);
    println!("{s_str_all0}");
    println!("{s_str_all1}");
    println!("{s_str_all2}");
    println!("{hello}");
    println!("{:?}", hello_get);

    // error[E0277]: the type `str` cannot be indexed by `{integer}`
    // let hello0 = &s_str_all0[0]; // 字符串切片不支持索引操作

    // panicked:byte index 14 is not a char boundary; it is inside '中' (bytes 12..15) of `hello world 中国`
    let _china_str = &s_str_all0[14..15]; // 索引操作(usize类型)
}
