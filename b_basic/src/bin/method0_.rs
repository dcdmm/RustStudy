// 方法

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    /*
     The &self is actually short for self: &Self.
     Within an impl block, the type Self is an alias for the type that the impl block is for.
     Methods must have a parameter named self of type Self for their first parameter

    Methods can take ownership of self, borrow self immutably, as we’ve done here, or borrow self mutably, just as they can any other parameter.
    * `self`: 拥有所有权
    * `&self`: 不可变借用
    * `&mut self`: 可变借用
     */

    // self: &Rectangel <==等价于==> self: &Self <==等价于==> &self
    fn area(self: &Rectangle) -> u32 {
        self.width * self.height
    }

    // Each struct is allowed to have multiple impl blocks
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

    // The method syntax goes after an instance: we add a dot followed by the method name, parentheses, and any arguments.
    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}", rect1.can_hold(&rect3));
}
