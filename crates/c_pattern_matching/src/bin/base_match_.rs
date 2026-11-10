// match

#[allow(warnings)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

#[allow(warnings)]
fn vic(coin: &Coin) -> f64 {
    let result = match coin {
        // 分支语法:模式 => 表达式(如果有多行代码,则需要用{}包裹)
        // 按顺序依次进行匹配
        // 必须穷尽所有可能
        // 所有分支表达式(match本身也是一个表达式)的返回类型必须一致
        Coin::Penny => 1.0,
        Coin::Nickel => 2.0,
        Coin::Dime => {
            println!("Lucky Dime!");
            3.0
        }
        Coin::Quarter => 4.0,
    };
    result
}

#[test]
fn t0() {
    let coin = Coin::Dime;
    let c0 = vic(&coin);
    println!("{}", c0);
}

#[test]
fn catch_all() {
    let x = 9;
    let u0 = match x {
        3 => 3,
        7 => 7,
        // This code compiles, even though we haven’t listed all the possible values a u8 can have, because the last pattern will match all values not specifically listed.
        other => other + 1, // 匹配任意i32类型的值并绑定(复制,i32实现了Copy trait)到局部变量other上
    };
    println!("{}", u0);

    let u1 = match x {
        3 => 3,
        7 => 7,
        // _ is a special pattern that matches any value and does not bind to that value.
        _ => -1,
    };
    println!("{}", u1);
}

// *****************************************************************
// matches!

/*
判断表达式的值"是否"匹配pattern,源码:
macro_rules! matches {
    ($expression:expr, $pattern:pat $(if $guard:expr)? $(,)?) => {
        #[allow(non_exhaustive_omitted_patterns)]
        match $expression {
            $pattern $(if $guard)? => true,
            _ => false
        }
    };
}
*/

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
}

#[test]
fn matches_t0() {
    let msg = Message::Write(String::from("hi"));

    let is_write = match msg {
        Message::Write(_) => true,
        _ => false,
    };

    let is_write_m = matches!(msg, Message::Write(_)); // 与上等价

    assert!(is_write && is_write_m);
}

#[test]
fn matches_t1() {
    // 配合|和范围模式
    let c = 'k';
    assert!(matches!(c, 'a'..='z' | 'A'..='Z'));
    assert!(!matches!(c, '0'..='9'));

    // 配合match guard
    let msg = Message::Move { x: 3, y: -1 };
    assert!(matches!(msg, Message::Move { x, .. } if x > 0));
}

#[test]
fn matches_t2() {
    // 配合迭代器使用

    let msgs = vec![
        Message::Quit,
        Message::Write(String::from("hi")),
        Message::Move { x: 1, y: 2 },
        Message::Quit,
    ];
    let quit_count = msgs.iter().filter(|m| matches!(m, Message::Quit)).count();
    assert_eq!(quit_count, 2);

    let s = "a1b2c3";
    let digits: String = s.chars().filter(|c| matches!(c, '0'..='9')).collect();
    assert_eq!(digits, "123");
}

fn main() {}
