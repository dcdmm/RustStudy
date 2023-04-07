// std::string::String

/*
A UTF-8–encoded, growable string.

The String type is the most common string type that has ownership over the contents of the string. It has a close relationship with its borrowed counterpart, the primitive str.
 */
fn main() {
    let s = String::from("hello world");
    // 报错:the type `String` cannot be indexed by `{integer}`
    // let _c = s[0];

    // You should use ranges to create string slices with caution, because doing so can crash your program.
    let ss = &s[0..4]; // 切片;每个英文字母占用1个字节(UTF-8编码中)
    println!("{}", ss);
    println!("{}", &String::from("你好世界")[0..3]); // 通常情况下,每个中文字占用3个字节(UTF-8编码中)
    // thread 'main' panicked at 'byte index 4 is not a char boundary; it is inside '好' (bytes 3..6) of `你好世界`'
    // println!("{}", &String::from("你好世界")[0..4]);


    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    /*
    The string s3 will contain Hello, world!. The reason s1 is no longer valid after the addition, and the reason we used a reference to s2, has to do with the signature of the method that’s called when we use the + operator.
    The + operator uses the add method, whose signature looks something like this:
    `fn add(self, s: &str) -> String {`
    In the standard library, you'll see add defined using generics and associated types.
    Here, we’ve substituted in concrete types, which is what happens when we call this method with String values.
     */
    let s3 = s1 + &s2; // note s1 has been moved here and can no longer be used
    println!("{s3}");

    // If we need to concatenate multiple strings, the behavior of the + operator gets unwieldy
    let s4 = String::from("tic");
    let s5 = String::from("tac");
    let s6 = String::from("toe");
    let s7 = s4 + "-" + &s5 + "-" + &s6;
    println!("{s7}");

    // 与上等价
    // 代码更加简洁
    let s8 = String::from("tic");
    let s9 = String::from("tac");
    let s10 = String::from("toe");
    let s_11 = format!("{s8}-{s9}-{s10}");
    println!("{s_11}");
}


#[test]
fn create() {
    // Creating a new, empty String
    let _s0 = String::new();

    let data = "initial contents";
    // Using the to_string method to create a String from a string literal
    let _s1 = data.to_string();

    // Using the String::from function to create a String from a string literal
    let _s2 = String::from("initial contents");
}

#[test]
fn iterating() {
    let s = String::from("hello, 我爱中国!");
    for i in s.chars() { // Returns an iterator over the chars of a string slice.
        println!("{i}")
    }
    for j in s.bytes() { // An iterator over the bytes of a string slice.
        println!("{j}")
    }
}
