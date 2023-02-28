// 模式匹配(结构枚举)

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

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        // Another useful feature of match arms is that they can bind to the parts of the values that match the pattern.
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}

fn main() {
    let c = Coin::Quarter(UsState::Alabama);
    println!("{}", value_in_cents(c))
}
