// 布尔类型(Primitive Type bool)与逻辑运算

// The boolean type or bool is a primitive data type that can take on one of two values, called true and false.

fn main() {
    let x = true;
    let y = false;
    println!("{}", y);
    println!("{}", !x); // print->false
    println!("{}", x | y); // print->true
    println!("{}", x & y); // print->false
}
