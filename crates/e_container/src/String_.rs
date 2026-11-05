// std::string::String

/*
A UTF-8–encoded, growable string.

The String type is the most common string type that has ownership over the contents of the string. It has a close relationship with its borrowed counterpart, the primitive str.
 */

#[test]
fn t0() {
    let mut s0 = String::new();
    s0.push('1');
    println!("{}", s0);

    let mut s1 = String::from("hello jave");
    let msl = s1.as_mut_str();
    let ms = String::from(msl);
    let s2 = String::from('s');
    let s3 = String::from(&s1);
    let bs = "Hello, world!".to_owned().into_boxed_str();
    let s4 = String::from(bs);
}

#[test]
fn t1() {
    let s = String::from("hello world");
    // error[E0277]: the type `String` cannot be indexed by `{integer}`
    // let _c = s[0];

    // You should use ranges to create string slices with caution, because doing so can crash your program.
    let ss = &s[0..4]; // 切片;每个英文字母占用1个字节(UTF-8编码中)
    println!("{}", ss); // print->hell
    println!("{}", &String::from("你好世界")[0..3]); // print->你;通常情况下,每个中文字占用3个字节(UTF-8编码中)

    // panicked:byte index 4 is not a char boundary; it is inside '好' (bytes 3..6) of `你好世界`
    println!("{}", &String::from("你好世界")[0..4]);
}

#[test]
fn add_test() {
    let s0 = String::from("hello ");
    let s1 = String::from("world ");
    /*
    String的Add特质实现如下:
    impl Add<&str> for String {
        type Output = String;

        #[inline]
        fn add(mut self, other: &str) -> String {
            self.push_str(other);
            self
        }
    }
     */
    let s2 = s0 + &s1; // `s0`: value moved here
    println!("{}", s2); // print->hello world
    println!("{}", s1);

    // error[E0382]: borrow of moved value: `s0`
    // println!("{}", s0);
}
