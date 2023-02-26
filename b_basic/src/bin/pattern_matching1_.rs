// 模式匹配

#[derive(Debug)]
#[allow(dead_code)]
enum UsState {
    Alabama,
    Alaska,
}

#[allow(dead_code)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}
fn main() {
    let config_max = Some(3u8);
    
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }

    // 与上等价
    // Using if let means less typing, less indentation, and less boilerplate code. However, you lose the exhaustive checking that match enforces. 
    // Choosing between match and if let depends on what you’re doing in your particular situation and whether gaining conciseness is an appropriate trade-off for losing exhaustive checking.

    // In other words, you can think of if let as syntax sugar for a match that runs code when the value matches one pattern and then ignores all other values.
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }

    // We can include an else with an if let. The block of code that goes with the else is the same as the block of code that would go with the _ case in the match expression that is equivalent to the if let and else. 
    let coin = Coin::Quarter(UsState::Alabama);
    match coin {
        Coin::Quarter(ref state) => println!("State quarter from {:?}!", state),
        _ => (),
    }
    // 与上等价
    if let Coin::Quarter(ref state) = coin {
        println!("State quarter from {:?}!", state);
    } else {
        ();
    }
}