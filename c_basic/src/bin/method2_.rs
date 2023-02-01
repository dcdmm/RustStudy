// Multiple impl Blocks

struct Rectangle {
    width: u32,
    height: u32,
}

// Multiple impl Blocks
impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
}

// Multiple impl Blocks
impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        rect1.area()
    );

    println!("Can rect1 hold rect2? {}", rect1.can_hold(&rect2));
}
