// 模式匹配(仅match)(Extra Conditionals with Match Guards)

fn main() {
    let num = Some(4);
    match num {
        Some(x) if x % 2 == 0 => println!("The number {} is even", x),
        Some(x) => println!("The number {} is odd", x),
        None => (),
    }
    // println!("{x}"); // x仅在对应match分支内有效

    
    let a = 4;
    let b = false;
    match a {
        4 | 5 | 6 if b => println!("yes"), // 优先级顺序为:(4 | 5 | 6) if y
        _ => println!("no"),
    }
}
