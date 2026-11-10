// matches!

/*
matches!(EXPRESSION, PATTERN)
matches!(EXPRESSION, PATTERN if GUARD)

判断表达式的值是否匹配pattern,返回bool,等价于:
match EXPRESSION {
    PATTERN => true,
    _ => false,
}
*/

#[allow(warnings)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
}

#[test]
fn t0() {
    let msg = Message::Write(String::from("hi"));

    let is_write = match msg {
        Message::Write(_) => true,
        _ => false,
    };

    // 与上等价
    let is_write_m = matches!(msg, Message::Write(_)); // 只判断变体,不关心内容
    assert!(is_write && is_write_m);

    // // error[E0369]: binary operation `==` cannot be applied to type `Message`
    // let is_quit = msg == Message::Quit; // Message没有实现PartialEq trait

    let is_quit = matches!(msg, Message::Quit); // matches!不要求类型实现PartialEq trait
    assert!(!is_quit);
}

#[test]
fn t1() {
    // 配合|和范围模式

    let c = 'k';
    assert!(matches!(c, 'a'..='z' | 'A'..='Z'));
    assert!(!matches!(c, '0'..='9'));

    // 配合match guard

    let msg = Message::Move { x: 3, y: -1 };
    assert!(matches!(msg, Message::Move { x, .. } if x > 0));

    // // error[E0425]: cannot find value `x` in this scope
    // println!("{x}"); // pattern中绑定的变量只能在guard中使用
}

#[test]
fn t2() {
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
