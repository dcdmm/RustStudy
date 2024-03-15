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
    println!("{}", pr.x); // 自动解引用(.运算符)
    let prr: &&Point = &pr;
    println!("{}", prr.x); // 自动解引用(.运算符)
}
