// 模式匹配(match/Matching Named Variables)

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
fn matching_named_variables() {
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        /*
        The pattern in the second match arm introduces a new variable named y that will match any value inside a Some value. Because we’re in a new scope inside the match expression, this is a new y variable, not the y we declared at the beginning with the value 10. This new y binding will match any value inside a Some, which is what we have in x.
        Therefore, this new y binds to the inner value of the Some in x. That value is 5, so the expression for that arm executes and prints Matched, y = 5.
         */
        Some(y) => println!("Matched, y = {y}"), // print->Matched, y = 5

        // If x had been a None value instead of Some(5), the patterns in the first two arms wouldn’t have matched, so the value would have matched to the underscore.
        _ => println!("Default case, x = {:?}", x),
    }

    // When the match expression is done, its scope ends, and so does the scope of the inner y.
    println!("at the end: x = {:?}, y = {y}", x); // print->at the end: x = Some(5), y = 10
}

#[test]
fn catch_all() {
    let x = 9;
    let u0 = match x {
        3 => 3,
        7 => 7,
        // This code compiles, even though we haven’t listed all the possible values a u8 can have, because the last pattern will match all values not specifically listed.
        other => other + 1, // 匹配的值绑定到变量other上
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
