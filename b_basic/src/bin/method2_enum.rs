// 为枚举实现方法

#[allow(warnings)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String)
}

impl Message {
    fn call(&self) {
        match self {
            Message::Quit => println!("Quit"),
            Message::Move { x, y } => println!("Move to ({}, {})", x, y),
            Message::Write(text) => println!("Text message: {}", text),
        }
    }
}

fn main() {
    let m = Message::Write(String::from("hello"));
    m.call();
}
