// 变量

fn main() {
    let x = 5; // 变量在默认情况下是不可变的
    println!("The value of x is: {}", x);

    // error[E0384]: cannot assign twice to immutable variable `x`
    // x = 6;

    let mut y = 5; // 定义可变变量(变量名前加`mut`关键字)
    println!("The value of y is: {}", y);
    y = 6;
    println!("The value of y is: {}", y);

    let z: i32; // 变量(未初始化)
    // error[E0381]: used binding `z` is possibly-uninitialized
    // println!("The value of z is: {}", z);
    
    z = 5;
    println!("The value of z is: {}", z);
}
