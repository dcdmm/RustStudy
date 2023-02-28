// 模式匹配(Extra Conditionals with Match Guards)

fn main() {
    let num = Some(5);

    match num {
        Some(x) if x % 2 == 0 => println!("The number {} is even", x),
        Some(x) => println!("The number {} is odd", x),
        None => (),
    }

    let x = 4;
    let y = false;

    match x {
        // 优先级为:(4 | 5 | 6) if y
        4 | 5 | 6 if y => println!("yes"),
        _ => println!("no"),
    }
}
