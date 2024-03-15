// 自动解引用(.运算符)

#[allow(warnings)]
#[derive(Debug)]
struct Point {
    x: String,
    y: i32,
}

#[allow(warnings)]
struct Color(i32, Point);

fn main() {
    let p = Point {
        x: String::from("hello"),
        y: 3,
    };
    let pr = &p;
    println!("{}", pr.x); // 隐式解引用(dot(.)运算符按需对左操作数隐式引用和解引用)
    let prr: &&Point = &pr;
    println!("{}", prr.x); // 隐式解引用(dot(.)运算符按需对左操作数隐式引用和解引用)
}
