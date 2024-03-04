// 条件选择结构:if/else表达式

fn main() {
    let x = 5;
    if x > 4 {
        println!("x大于4")
    }

    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

    let condition = true;
    let number = if condition { 
        println!("{}", "5!");
        5 
    } else { 
        println!("{}", "6!"); // 语句
        6 // 表达式
    }; // 类似python/c++/Java三元表达式
    println!("{}", number);

    // This won’t work because variables must have a single type, and Rust needs to know at compile time what type the number variable is, definitively.
    // error[E0308]: `if` and `else` have incompatible types
    // let number_1 = if condition { 5 } else { "sex" };

    // error[E0317]: `if` may be missing an `else` clause
    // if` expressions without `else` evaluate to `()`
    // let number_2 = if condition {5};
}
