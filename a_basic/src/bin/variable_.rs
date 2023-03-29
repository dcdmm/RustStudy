// 变量

fn main() {
    let x = 5; // 变量在默认情况下是不可变的
    println!("The value of x is: {}", x);

    // 报错:cannot assign twice to immutable variable
    // x = 6;

    let mut y = 5; // 定义可变变量(变量前加`mut`关键字)
    println!("The value of y is: {}", y);
    y = 6;
    println!("The value of y is: {}", y);

    let z: i32; // 变量(未初始化)
    // 报错:used here but it isn't initialized
    // println!("The value of z is: {}", z);
    
    z = 5;
    println!("The value of z is: {}", z);
}
