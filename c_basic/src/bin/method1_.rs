// Associated Functions(含义类似python __new__(cls[, ...]) 魔法方法)

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }

    // The Self keywords in the return type and in the body of the function are aliases for the type that appears after the impl keyword, which in this case is Rectangle.
    // 不将`self`作为第一个参数(因此不是方法,称为Associated Functions,即关联函数),故不能通过xxx.read()的方式调用
    fn square(size: u32) -> Self { // 等价于:fn square(size: u32) -> Rectangle {
        Self { // 等价于:Rectangle {
            width: size,
            height: size,
        }
    }
}

fn main() {
    // To call this associated function, we use the :: syntax with the struct name
    let sq = Rectangle::square(3);
    println!("The area of the square is {} square pixels.", sq.area());
}
