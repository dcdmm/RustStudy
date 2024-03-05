// 模式匹配(if let;ref关键字)

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

#[test]
fn t0() {
    let config_max = Some(3u8);

    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }

    // 与上等价
    /*
    Using if let means less typing, less indentation, and less boilerplate code. However, you lose the exhaustive checking that match enforces.
    Choosing between match and if let depends on what you’re doing in your particular situation and whether gaining conciseness is an appropriate trade-off for losing exhaustive checking.
    In other words, you can think of if let as syntax sugar for a match that runs code when the value matches one pattern and then ignores all other values.
     */
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }

    // **************************************************************
    // We can include an else with an if let. The block of code that goes with the else is the same as the block of code that would go with the _ case in the match expression that is equivalent to the if let and else.
    let coin = Coin::Quarter(UsState::Alabama);
    /*
    Bind by reference during pattern matching.

    ref annotates pattern bindings to make them borrow rather than move. It is not a part of the pattern as far as matching is concerned: it does not affect whether a value is matched, only how it is matched.
    Using the ref keyword, the value is only borrowed, not moved, making it available for use after the match statement:

    & vs ref
    * & denotes that your pattern expects a reference to an object. Hence & is a part of said pattern: &Foo matches different objects than Foo does.
    * ref indicates that you want a reference to an unpacked value. It is not matched against: Foo(ref foo) matches the same objects as Foo(foo).
     */
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

    let s1 = String::from("hello world");
    let s1_0 = s1;
    // 报错: borrow of moved value: `s1`
    // println!("s1:{}", s1);
    println!("s1_0:{}", s1_0);

    let s2 = String::from("hello rust");
    let s2_0 = &s2;
    println!("s2:{}", s2);
    println!("s2_0:{}", s2_0);

    let s3 = String::from("hello rust");
    let ref s3_0 = s2;
    println!("s3:{}", s3);
    println!("s3_0:{}", s3_0);
}
