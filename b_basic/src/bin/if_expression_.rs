// 条件选择结构:if/else语句

fn main() {
    let x = 5;
    if x > 4{
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
    let number = if condition {5} else {6};  // 类似python/Java三元表达式
    
    // 报错:expected integer, found `&str`
    // This won’t work because variables must have a single type, and Rust needs to know at compile time what type the number variable is, definitively. 
    // let number = if condition {5} else {"sex"};
    
    println!("{}", number)

}