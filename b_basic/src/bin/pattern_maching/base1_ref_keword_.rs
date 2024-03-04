// 模式匹配(if let;ref关键字)

#[allow(warnings)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

#[test]
fn t0() {
    // let coin = Coin::Dime;

    // match a {
    //     Some(v) => println!("{}", v),
    //     _ => println!("{}", "none"),
    // }
    // 与上等价
    // you can think of if let as syntax sugar for a match that runs code when the value matches one pattern and then ignores all other values.
    // if let Some(max) = config_max {
    //     println!("{}", max);
    // }

    // We can include an else with an if let. The block of code that goes with the else is the same as the block of code that would go with the _ case in the match expression that is equivalent to the if let and else.



    // let coin = Coin::Quarter(UsState::Alabama);
    // match coin {
    //     Coin::Quarter(state) => println!("State quarter from {:?}!", state),
    //     _ => (),
    // }
    // 与上等价
    // if let Coin::Quarter(state) = coin {
    //     println!("State quarter from {:?}!", state);
    // } else {
    //     ();
    // }

    // let s1 = String::from("hello world");
    // let s1_0 = s1;
    // // 报错: borrow of moved value: `s1`
    // // println!("s1:{}", s1);
    // println!("s1_0:{}", s1_0);

    // let s2 = String::from("hello rust");
    // let s2_0 = &s2;
    // println!("s2:{}", s2);
    // println!("s2_0:{}", s2_0);

    // let s3 = String::from("hello rust");
    // let ref s3_0 = s2;
    // println!("s3:{}", s3);
    // println!("s3_0:{}", s3_0);
    // // **************************************************************
    // let favorite_color: Option<&str> = None;
    // let is_tuesday = false;
    // let age: Result<u8, _> = "34".parse();

    // // it’s also possible to mix and match if let, else if, and else if let expressions
    // if let Some(color) = favorite_color {
    //     println!("Using your favorite color, {color}, as the background");
    // } else if is_tuesday {
    //     println!("Tuesday is green day!");
    // } else if let Ok(age) = age {
    //     if age > 30 {
    //         println!("Using purple as the background color");
    //     } else {
    //         println!("Using orange as the background color");
    //     }
    // } else {
    //     println!("Using blue as the background color");
    // }
}
