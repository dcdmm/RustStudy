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
        // The code associated with each arm is an expression, and the resultant value of the expression in the matching arm is the value that gets returned for the entire match expression.(返回值类型相同)
        Coin::Penny => 1.0,
        Coin::Nickel => 5.0,
        Coin::Dime => 10.0,
        Coin::Quarter => 25.0,
    };
    result
}

#[allow(dead_code)]
fn value_in_cents_placeholder(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        // _ is a special pattern that matches any value and does not bind to that value. This tells Rust we aren’t going to use the value, so Rust won’t warn us about an unused variable.
        _ => 99,
    }
}

fn main() {
    let coin = Coin::Nickel;
    let result = match coin {
        // We don’t typically use curly brackets if the match arm code is short
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

    // ***************************************************************************
    // Matching Named Variables
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        // The pattern in the second match arm introduces a new variable named y that will match any value inside a Some value. 
        // Because we’re in a new scope inside the match expression, this is a new y variable, not the y we declared at the beginning with the value 10. 
        // This new y binding will match any value inside a Some, which is what we have in x.
        Some(y) => println!("Matched, y = {y}"),
        _ => println!("Default case, x = {:?}", x),
    }

    // When the match expression is done, its scope ends, and so does the scope of the inner y.
    println!("at the end: x = {:?}, y = {y}", x);
}
