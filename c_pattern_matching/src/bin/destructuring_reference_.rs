// 解构引用(参考matching_named_variables_.rs)

#[allow(warnings)]
fn main() {
    struct Point {
        x: i32,
        y: i32,
    }
    let mut p = Point { x: 0, y: 7 };

    // 解构一个指向复合类型的引用时,解构的内容会保持引用的性质(包括可变性)
    let Point { x, y } = &mut p; // x类型为&mut i32
    let &Point { x: x_, y: y_ } = &p; // x_类型为i32

    let s = &mut Some(String::from("hello rust"));
    if let &mut Some(ref sx_) = s {} // ref sx_类型为&String
    if let Some(sx) = s {} // sx类型为&mut String

    let t = &mut (String::from("hello rust"), 1);
    let &mut (ref tx_, ref ty_) = t; // ref tx_类型为&String
    let (tx, ty) = t; // tx类型为&mut String

    let a = &mut [String::from("hello"), String::from("rust")];
    let &mut [ref ax_, ref ay_] = a; // ref ax_类型为&String
    let [ax, ay] = a; // ax类型为&mut String
}