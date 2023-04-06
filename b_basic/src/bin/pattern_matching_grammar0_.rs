// 模式匹配(Multiple Patterns;Matching Ranges of Values)

fn main() {
    let z = 1;

    // Multiple Patterns
    match z {
        // In match expressions, you can match multiple patterns using the | syntax, which is the pattern or operator.
        1 | 2 => println!("one or two"),
        3 => println!("three"),
        _ => println!("anything"),
    }

    // *******************************************************
    let s = 5;

    // Matching Ranges of Values with ..=
    match s {
        // The ..= syntax allows us to match to an inclusive range of values.

        // If x is 1, 2, 3, 4, or 5, the first arm will match.
        // This syntax is more convenient for multiple match values than using the | operator to express the same idea; if we were to use | we would have to specify 1 | 2 | 3 | 4 | 5.
        1..=5 => println!("one through five"),
        _ => println!("something else"),
    }

    let n = 'c';

    match n {
        'a'..='j' => println!("early ASCII letter"),
        'k'..='z' => println!("late ASCII letter"),
        _ => println!("something else"),
    }
}
