// 枚举

// defining an enum
enum IpAddrkind {
    V4,
    V6,
}

fn route(_ip_kind: IpAddrkind) {}

enum IpAddr {
    // can put data directly into each enum variant
    V4(String),
    V6(String),
}

enum IpAddr1 {
    // each variant can have different types and amounts of associated data.
    V4(u8, u16, u32, u64),
    V6(String),
}

// This code illustrates that you can put any kind of data inside an enum variant: strings, numeric types, or structs, for example. You can even include another enum
enum Message {
    Quit,                       // Quit has no data associated with it at all.
    Move { x: i32, y: i32 },    // Move has named fields like a struct does.
    Write(String),              // Write includes a single String.
    ChangeColor(i32, i32, i32), // ChangeColor includes three i32 values.
}

fn main() {
    // 通过`::`操作符来访问IpAddkind下的具体成员
    let _four = IpAddrkind::V4;
    let _six = IpAddrkind::V6;

    route(IpAddrkind::V4);
    route(IpAddrkind::V6);

    let _four_funciton: fn(String) -> IpAddr = IpAddr::V4; // 类型为:fn(String) -> Ipaddr
    let _four_obj: IpAddr = _four_funciton(String::from("192.168.2.12"));
    let _home = IpAddr::V4(String::from("127.0.0.1"));
    let _loopback = IpAddr::V6(String::from("::1"));
    let _home1 = IpAddr1::V4(127, 0, 0, 1);
    let _loopback1 = IpAddr1::V6(String::from("::1"));

    let _m1 = Message::Quit;
    let _m2 = Message::Move { x: 2, y: 3 };
    let _m3 = Message::Write(String::from(""));
    let _m4 = Message::ChangeColor(1, 2, 3);
}
