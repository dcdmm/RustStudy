// 枚举 

// defining an enum
enum IpAddrkind {
    V4,
    V6,    
}

fn route(_ip_kind: IpAddrkind) {}

fn main() {
    // 通过`::`操作符来访问IpAddkind下的具体成员
    let _four = IpAddrkind::V4;
    let _six = IpAddrkind::V6;

    route(IpAddrkind::V4);
    route(IpAddrkind::V6);
}