// match模式匹配语法(Multiple Patterns;Matching Ranges of Values)

#[test]
fn t0() {
    // Multiple Patterns

    let z = 1;
    match z {
        // In match expressions, you can match multiple patterns using the | syntax, which is the pattern or operator.
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    if let 1 | 2 = z {
        println!("one or two")
    } else if let 3 = z {
        println!("three")
    } else {
        println!("anything")
    }

    // *******************************************************
    // Matching Ranges of Values with ..=

    let s = -1;
    match s {
        // The ..= syntax allows us to match to an inclusive range of values.

        /*
        If x is 1, 2, 3, 4, or 5, the first arm will match. This syntax is more convenient for multiple match values than using the | operator to express the same idea; if we were to use | we would have to specify 1 | 2 | 3 | 4 | 5.
         */
        ..=0 => println!("in [MIN, 0]"),  // 匹配[i32最小值, 0]
        1..=5 => println!("in [1, 5]"),   // 匹配[1, 5]
        10.. => println!("in [10, MAX]"), // 匹配[10, i32最大值]
        _ => println!("something else"),
    }

    if let ..=0 = s {
        println!("in [MIN, 0]")
    } else if let 1..=5 = s {
        println!("in [1, 5]")
    } else if let 10.. = s {
        println!("in [10, MAX]")
    } else {
        println!("something else")
    }

    let n = 'c';
    match n {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }
}
