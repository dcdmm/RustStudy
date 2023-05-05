// 泛型
/*
You might be wondering whether there is a runtime cost when using generic type parameters.
The good news is that using generic types won't make your program run any slower than it would with concrete types.

Rust accomplishes this by performing monomorphization of the code using generics at compile time.
Monomorphization is the process of turning generic code into specific code by filling in the concrete types that are used when compiled.

Let’s look at how this works by using the standard library’s generic Option<T> enum:

```rust
let integer = Some(5);
let float = Some(5.0);
```

When Rust compiles this code, it performs monomorphization.
During that process, the compiler reads the values that have been used in Option<T> instances and identifies two kinds of Option<T>: one is i32 and the other is f64. As such, it expands the generic definition of Option<T> into two definitions specialized to i32 and f64, thereby replacing the generic definition with the specific ones.
 */

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
