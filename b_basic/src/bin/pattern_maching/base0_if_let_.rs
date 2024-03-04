// 模式匹配(if let)

#[allow(warnings)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

#[test]
fn t0() {
    let coin = Coin::Penny;

    // you can think of if let as syntax sugar for a match that runs code when the value matches one pattern and then ignores all other values.
    if let Coin::Penny = coin {
        println!("{}", "Penny") 
    }
    match coin {
        Coin::Penny => println!("{}", "Penny"),
        _ => println!("{}", "other"),
    }

    // 与上等价
    // We can include an else with an if let. The block of code that goes with the else is the same as the block of code that would go with the _ case in the match expression that is equivalent to the if let and else. 
    if let Coin::Penny = coin {
        println!("{}", "Penny") 
    } else {
        println!("{}", "other") 
    }
}