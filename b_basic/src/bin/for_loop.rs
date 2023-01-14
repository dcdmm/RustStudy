// for循环

fn main() {
    for i in 1..5 {
        println!("{i}")
    }
    
    // 警告:warning: unused variable: `i`
    // for i in 1..5 { 
    for _ in 1..5 { 
        println!("hello rust!")
    }

    let v = &["apples", "cake", "coffee"];

    for text in v {
        println!("I like {}.", text);
    }
}
