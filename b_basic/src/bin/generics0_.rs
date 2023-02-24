// 泛型

// In Function Definitions
fn add<T: std::ops::Add<Output = T>>(a: T, b: T) -> T {
    a + b
}

// In Struct Definitions
#[allow(dead_code)]
struct Point<T> {
    // fields x and y are both that same type, whatever that type may be
    x: T,
    y: T,
}

#[allow(dead_code)]
struct Point1<T, U> {
    // x and y are both generics but could have different types, we can use multiple generic type parameters
    x: T,
    y: U
}

// In Enum Definitions
#[allow(dead_code)]
enum Option<T> { // the Option<T> enum that the standard library provides
    Some(T),
    None,
}

#[allow(dead_code)]
enum Result<T, E> { // Enums can use multiple generic types as well
    Ok(T),
    Err(E),
}

fn main() {
    println!("add i8: {}", add(2i8, 3i8));
    println!("add i32: {}", add(20, 30));
    println!("add f32:{}", add(1.23, 1.3));

    let _integer = Point {x: 5, y:10};
    let _float = Point{x:1.0, y: 4.0};
    
    let _integer1 = Point1 {x: 5, y:10};
    let _float1 = Point1{x:1, y: 4.0};
}
