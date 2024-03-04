// 模式匹配(match)

#[allow(warnings)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

#[allow(warnings)]
fn value_in_cents(coin: &Coin) -> f64 {
    let result = match coin {
        // 按顺序依次进行匹配
        // 必须穷尽所有可能
        // 与每个分支相关联的代码是一个表达式(如果有多行代码,则需要用{}包裹)
        // 所有分支(match本身也是一个表达式)的返回类型必须一致
        Coin::Penny => 34.1,
        Coin::Nickel => 5.0,
        Coin::Dime => {
            println!("Lucky penny!");
            10.0
        }
        Coin::Quarter => 25.0,
    };
    result
}

#[allow(warnings)]
fn value_other() -> u8 {
    fn add_fancy_hat(number: u8) -> u8 {
        number
    }
    fn remove_fancy_hat(number: u8) -> u8 {
        number
    }
    fn move_player(number: u8) -> u8 {
        number
    }

    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(3),
        7 => remove_fancy_hat(7),
        // This code compiles, even though we haven’t listed all the possible values a u8 can have, because the last pattern will match all values not specifically listed.
        other => move_player(other), // The code that runs for the other arm uses the variable by passing it to the move_player function.
    }
}

#[allow(warnings)]
fn value_in_cents_placeholder(coin: &Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        // _ is a special pattern that matches any value and does not bind to that value.
        _ => 99,
    }
}

#[test]
fn t0() {
    let coin = Coin::Dime;
    let c0 = value_in_cents(&coin);
    println!("{}", c0);

    let u0 = value_other();
    println!("{}", u0);

    let c1 = value_in_cents_placeholder(&coin);
    println!("{}", c1);
}
