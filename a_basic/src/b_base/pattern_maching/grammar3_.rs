// 模式匹配(@ Bindings)

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

#[test]
fn t0() {
    enum Message {
        Hello { id: i32 },
    }

    let msg = Message::Hello { id: 5 };

    match msg {
        // The at operator @ lets us create a variable that holds a value at the same time as we’re testing that value for a pattern match.
        Message::Hello {
            id: id_variable @ 3..=7,
        } => println!("Found an id in range: {}", id_variable),
        Message::Hello { id: 10..=12 } => {
            // 无法使用字段中的变量`id`
            println!("Found an id in another range")
        }
        Message::Hello { id } => println!("Found some other id: {}", id),
    }
    
    // 绑定新变量`p`,同时对Point进行解构
    let p @ Point {x: px, y: py } = Point {x: 10, y: 23};
    println!("x: {}, y: {}", px, py);
    println!("{:?}", p);
}
