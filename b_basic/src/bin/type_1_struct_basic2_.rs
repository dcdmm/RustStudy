// tuple structs

struct Color(i32, f32, i32); // 拥有3个字段,字段没有名称(★★★★★且以分号结尾)

struct Point(i32, f32, f64);

fn main() {
    let black = Color(0, 0.0, 0);
    let _origin = Point(0, 0.0, 0.0);
    // tuple struct instances are similar to tuples in that you can destructure them into their individual pieces, and you can use a . followed by the index to access an individual value.
    let x = black.0; // 通过索引访问结构体的值
    let y = black.1;
    let z = black.2;
    println!("{}", x);
    println!("{}", y);
    println!("{}", z);
}