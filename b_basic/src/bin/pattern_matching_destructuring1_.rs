// 模式匹配(结构结构体)

struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p = Point { x: 0, y: 7 };
    
    let Point { x: a, y: b } = p;
    println!("a:{}", a);
    println!("b:{}", b);
    
    let Point { x, y } = p; // 简写(与上等价)
    println!("x:{}", x);
    println!("y:{}", y);


    let p = Point { x: 0, y: 7 };

    match p {
        // 匹配y为0的Point
        Point { x, y: 0 } => println!("On the x axis at {x}"),
        // 匹配x为0的Point
        Point { x: 0, y } => println!("On the y axis at {y}"),
        // 匹配任意Point
        Point { x, y } => {
            println!("On neither axis: ({x}, {y})");
        }
    }
}