// 模式匹配语法(@ Bindings)

#[test]
fn t0() {
    enum Message {
        Hello { id: i32 },
    }

    let msg = Message::Hello { id: 5 };

    match msg {
        // The at operator @ lets us create a variable that holds a value at the same time as we’re testing that value for a pattern match.
        Message::Hello {
            // 匹配结构体字段id的值是否位于[3, 7]范围内并将值绑定到局部变量id_variable上
            id: id_variable @ 3..=7,
        } => println!("Found an id in range: {}", id_variable),
        Message::Hello {
            // 结构体字段id的值是否位于[10, 12]范围内
            id: 10..=12,
        } => {
            println!("Found an id in another range")
        }
        Message::Hello {
            // 结构体字段id的值为任意范围
            // Message::Hello{id}为Message::Hello{id:id}的简写(见destructuring_struct.rs)
            id,
        } => println!("Found some other id: {}", id),
    }

    // 与上等价
    if let Message::Hello { id: id_var @ 3..=7 } = msg {
        println!("Found an id in range: {}", id_var)
    } else if let Message::Hello { id: 10..=12 } = msg {
        println!("Found an id in another range")
    } else {
        let Message::Hello { id } = msg;
        println!("Found some other id: {}", id)
    }
}

#[test]
fn t1() {
    #[derive(Debug)]
    #[allow(warnings)]
    struct Point {
        x: i32,
        y: i32,
    }

    let point = Point { x: 10, y: 20 };
    // 绑定新变量p,同时解构结构体
    let p @ Point { x, y } = point;
    println!("{}, {}", x, y);
    println!("{:?}", p);

    let points = vec![Point { x: 10, y: 20 }, Point { x: 15, y: 25 }];
    for ps @ Point { x, y } in points {
        println!("{}, {}", x, y);
        println!("Processing a point: {:?}", ps);
    }
}

#[test]
fn t2() {
    #[derive(Debug)]
    enum Message {
        Hello { id: i32 },
    }

    let mut messages = vec![
        Message::Hello { id: 3 },
        Message::Hello { id: 12 },
        Message::Hello { id: 10 },
        Message::Hello { id: 7 },
    ];
    // 绑定新变量m,同时解构Option<Message>,故变量m前需要添加关键字ref
    while let ref m @ Some(Message::Hello {
        id: id_var @ 3..=10,
    }) = messages.pop()
    {
        println!("{:?}", m);
        println!("{}", id_var);
    }

    let mut messages1 = vec![
        Message::Hello { id: 3 },
        Message::Hello { id: 12 },
        Message::Hello { id: 10 },
        Message::Hello { id: 7 },
    ];
    // 绑定新变量n,同时解构Option<Message>,故变量n和变量s前需要添加关键字ref
    while let ref n @ Some(ref s) = messages1.pop()
    {
        println!("{:?}", n);
        println!("{:?}", s);
    }
}
