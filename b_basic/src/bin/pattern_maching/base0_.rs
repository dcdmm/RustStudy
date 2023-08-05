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
        /*
        Rust also has a pattern we can use when we want a catch-all but don’t want to use the value in the catch-all pattern:
        _ is a special pattern that matches any value and does not bind to that value.
        This tells Rust we aren’t going to use the value, so Rust won’t warn us about an unused variable.
         */
        _ => 99,
    }
}

#[allow(dead_code)]
fn value_other() -> u8 {
    fn add_fancy_hat(number: u8) -> u8 { number }
    fn remove_fancy_hat(number: u8) -> u8 { number }
    fn move_player(number: u8) -> u8 { number }

    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(3),
        7 => remove_fancy_hat(7),
        // This code compiles, even though we haven’t listed all the possible values a u8 can have, because the last pattern will match all values not specifically listed.
        other => move_player(other)
    }
}

#[test]
fn t0() {
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