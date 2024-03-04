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
        other => other + 1, // Matching Named Variables;匹配任意值(i32类型)并绑定到变量other上
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
