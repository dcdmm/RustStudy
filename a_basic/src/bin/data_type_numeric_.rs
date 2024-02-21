// 数值类型(Primitive Type)与算术运算,赋值运算,比较运算

fn main() {
    let _x = 5; // 整型默认使用`i32`类型

    let _f0 = 5.34; // 浮点型默认使用`f64`类型
    let _f1: f32 = 5.34;
    let f2 = 5.34e2; // 科学计数法
    println!("{}", f2);

    // 算法运算符: +, -, *, /, %
    println!("{}", 5 + 10); // 加法
    println!("{}", 5 / 10); // 除法
    println!("{}", 11 / 5); // 整数与整数相除仍为整数
    println!("{}", 43 % 6); // 取余

    let mut x = 5;
    // 赋值运算符:+=, -=, *=, /=, %=
    x += 1;
    println!("{}", x);

    // 比较运算符:>, >=, <, <=, ==, !=
    println!("{}", 5 != 34); // print->true
    println!("{}", 5 > 5);
}