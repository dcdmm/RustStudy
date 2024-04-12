#![allow(warnings)]

// 解构枚举

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
}

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
        Coin::Quarter(UsState::Alabama) => 15,
        Coin::Quarter(x) => {
            println!("State quarter from {:?}!", x);
            25
        }
    }
}

fn main() {
    let c = Coin::Quarter(UsState::Alabama);
    println!("{}", value_in_cents(c)); // print->15

    let d = Coin::Quarter(UsState::Alaska);
    println!("{}", value_in_cents(d)) // print->25
}
