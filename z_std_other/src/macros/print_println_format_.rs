// Macro std::format
// Macro std::println
// Macro std::print

#[test]
fn base_test() {
    // Creates a String using interpolation of runtime expressions.
    let sf = format!("{a} {c} {b}", a = 1, b = 'b', c = 3);
    println!("{}", sf);
    // Prints to the standard output, with a newline.This macro uses the same syntax as format!, but writes to the standard output instead.
    println!("{a} {c} {b}", a = 1, b = 'b', c = 3);
    // Equivalent to the println! macro except that a newline is not printed at the end of the message.
    print!("{a} {c} {b}", a = 1, b = 'b', c = 3);
}

#[test]
fn t0() {
    println!("{}", format!("dmm")); // print-> dmm
    println!("{}", format!("{}/{}", "hello", "world")); // print->hello/world
    println!("{}", format!("{1}/{0}/{1}", "hello", "world")); // print->world/hello/world
    println!("{}", format!("{argument}", argument = "hello")); // print->hello
    println!("{}", format!("{name} {}", 1, name = "2")); // print->2 1
    println!("{}", format!("{a} {c} {b}", a = 1, b = 'b', c = 3)); // print->1 3 b

    let argument = 2 + 2;
    // If a named parameter does not appear in the argument list, format! will reference a variable with that name in the current scope.
    println!("{}", format!("{argument}")); // print->4
    fn make_string(a: u32, b: &str) -> String {
        format!("{b} {a}")
    }
    println!("{}", make_string(927, "label")); // print->label 927
}

#[test]
fn t1() {
    println!("{:4}", 5);
    println!("{:>4}", 5); // 与上等价(右对其,空白字符填充,总宽度为5)
    println!("{:<4}", 5); // 左对齐(空白字符填充,总宽度为5)
    println!("{:0<4}", 5); // 左对齐('0'字符填充,总宽度为5)
    println!("{:x<4}", 5); // 左对其('x'字符填充,总宽度为5)
    println!("{:x^4}", 5); // 居中对其('x'字符填充,总宽度为5)
    println!("{:x^4}", "A");
}

#[test]
fn t2() {
    let s = 3.1415926535;

    println!("{:}", s);
    println!("{:.4}", s); // 保留小数点后四位
    println!("{:x<10.4}", s); // 左对齐('x'字符填充,总宽度10,保留小数点后4位)
    println!("{:+.2}", s); // 带符号保留小数点后两位

    println!("{:5.2}", 3.14159); // 保留小数点后两位(总宽度为5,小数点符号'.'占一位)
    println!("{:5.2}", 33.14159);
    println!("{:5.2}", 333.14159);

    println!("{:e}", 100000000);
    println!("{:.2e}", 1000000000); // 科学计数法(保留2位有效数字)
}

#[test]
fn t3() {
    println!("{:b}", 11); // 二进制
    println!("{:o}", 11); //八进制
    println!("{}", 11); // 十进制
    println!("{:x}", 11); // 十六进制
}

#[test]
fn debug_test() {
    /*
    fmt::Display implementations assert that the type can be faithfully represented as a UTF-8 string at all times. It is not expected that all types implement the Display trait.

    fmt::Debug implementations should be implemented for all public types. Output will typically represent the internal state as faithfully as possible. The purpose of the Debug trait is to facilitate debugging Rust code. In most cases, using #[derive(Debug)] is sufficient and recommended.
    */
    // error[E0277]: `({integer}, {integer})` doesn't implement `std::fmt::Display`
    // println!("{}", (3, 4));
    println!("{:?}", (3, 4)); // print->(3, 4)
                              /*
                              print->
                              (
                                  3,
                                  4,
                              )
                               */
    println!("{:#?}", (3, 4));
}
