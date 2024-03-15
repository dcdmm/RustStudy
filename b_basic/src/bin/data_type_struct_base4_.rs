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
    let pr = &p; // pr类型为:&Point
    println!("{}", pr.x); // 自动解引用(.运算符)
    let prr = &pr; // prr类型为:&&Point
    println!("{}", prr.x); // 自动解引用(.运算符)

    let c = (84, p);
    let cr = &c;
    println!("{}", cr.0); // 自动解引用(.运算符)
    println!("{:?}", cr.1); // 自动解引用(.运算符)
    let crr = &cr;
    println!("{}", crr.0); // 自动解引用(.运算符)
    println!("{:?}", crr.1); // 自动解引用(.运算符)
}
