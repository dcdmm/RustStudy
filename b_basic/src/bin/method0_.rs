// 方法

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // `self`指代此类型的实例(本例中self指代的是Rectangle结构体实例)
    
    // `self`的使用跟函数参数一样,要严格遵守Rust的所有权规则
    // * `self`表示Rectangle的所有权转移到该方法中
    // * `&self`表示该方法对Rectangle的不可变借用
    // * `&mut self`表示可变借用
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    let rect3 = Rectangle {
        width: 60,
        height: 45,
    };

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}
