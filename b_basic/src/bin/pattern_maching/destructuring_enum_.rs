// 解构枚举

#[derive(Debug)]
#[allow(warnings)]
enum UsState {
    Alabama,
    Alaska,
}

#[allow(warnings)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

#[allow(warnings)]
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

#[test]
fn t0() {
    let c = Coin::Quarter(UsState::Alabama);
    println!("{}", value_in_cents(c)); // print->15 

    let d = Coin::Quarter(UsState::Alaska);
    println!("{}", value_in_cents(d)) // print->25
}
