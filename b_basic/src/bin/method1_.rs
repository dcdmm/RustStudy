// Associated Functions(作用类似python __new__(cls[, ...]) 魔法方法)

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    /*
    All functions defined within an impl block are called associated functions because they’re associated with the type named after the impl. We can define associated functions that don’t have self as their first parameter (and thus are not methods) because they don’t need an instance of the type to work with.

    The Self keywords in the return type and in the body of the function are aliases for the type that appears after the impl keyword, which in this case is Rectangle.
     */
    fn square(size: u32) -> Self {
        // ===本例中即===> fn square(size: u32) -> Rectangle {
        Self {
            // ===本例中即==> Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    // square是函数而不是方法,所以不能用.(即方法调用)的方式来调用,需要通过结构体名+Path expressions(::)来调用
    let sq: fn(u32) -> Rectangle = Rectangle::square; // 函数对象
    let sq = sq(3);

    println!("The area of the square is {} square pixels.", sq.area());
}
