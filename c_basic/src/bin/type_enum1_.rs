// 枚举

enum IpAddr {
    // can put data directly into each enum variant.
    // the name of each enum variant that we define also becomes a function that constructs an instance of the enum. 
    V4(String),
    V6(String),
}

enum IpAddr1 {
    // each variant can have different types and amounts of associated data. 
    V4(u8, u8, u8, u8),
    V6(String),
}

fn main() {
    let _four_funciton = IpAddr::V4; // 类型为:V4(String) -> IpAddr
    let _four_obj = _four_funciton(String::from("192.168.2.12")); // 类型为:IpAddr
    // IpAddr::V4() is a function call that takes a String argument and returns an instance of the IpAddr type.
    let _home = IpAddr::V4(String::from("127.0.0.1"));  // 类型为:IpAddr
    let _loopback = IpAddr::V6(String::from("::1"));
    
    let _home1 = IpAddr1::V4(127, 0, 0, 1);
    let _loopback1 = IpAddr1::V6(String::from("::1"));
}