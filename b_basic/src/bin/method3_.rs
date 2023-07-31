// 为枚举实现方法

#[allow(dead_code)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

impl Message {
    fn call(&self) {
        // method body would be defined here
    }
}

#[test]
fn t0() {
    let m = Message::Write(String::from("hello"));
    m.call();
}