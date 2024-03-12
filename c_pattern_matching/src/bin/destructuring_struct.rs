// 模式匹配(解构结构体)

#[allow(warnings)]
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p = Point { x: 0, y: 7 };

    let Point { x: a, y: b } = p;
    println!("a:{}", a);
    println!("b:{}", b);

    // 与上等价且更加简洁
    // you only need to list the name of the struct field, and the variables created from the pattern will have the same names.
    let Point { x, y } = p;
    println!("x:{}", x);
    println!("y:{}", y);

    let p1 = Point { x: 0, y: 7 };

    match p1 {
        // 匹配y为0的Point类型
        Point { x, y: 0 } => println!("On the x axis at {x}"),
        // 匹配x为0的Point类型
        Point { x: 0, y } => println!("On the y axis at {y}"),
        // 匹配任意Point类型
        Point { x, y } => {
            println!("On neither axis: ({x}, {y})");
        }
    }
}
