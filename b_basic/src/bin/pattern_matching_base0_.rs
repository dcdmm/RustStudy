// 模式匹配(match)

#[allow(dead_code)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

#[allow(dead_code)]
fn value_in_cents(coin: Coin) -> f64 {
    let result = match coin {
        // 按顺序依次进行匹配
        // 必须穷尽所有可能
        // The code associated with each arm is an expression, and the resultant value of the expression in the matching arm is the value that gets returned for the entire match expression.
        // `match` arms have incompatible types
        Coin::Penny => 34.1,
        Coin::Nickel => 5.0,
        Coin::Dime => 10.0,
        Coin::Quarter => 25.0,
    };
    result
}

#[allow(dead_code)]
fn value_in_cents_placeholder(coin: Coin) -> u8 {
    match coin {
        // We don’t typically use curly brackets if the match arm code is short
        Coin::Penny => 1,
        Coin::Nickel => 5,
        // _ is a special pattern that matches any value and does not bind to that value. This tells Rust we aren’t going to use the value, so Rust won’t warn us about an unused variable.
        _ => 99,
    }
}

fn main() {
    let coin = Coin::Nickel;
    let result = match coin {
        // If you want to run multiple lines of code in a match arm, you must use curly brackets, and the comma following the arm is then optional.
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    };
    println!("{}", result);
}
