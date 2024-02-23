// 方法

struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    /*
    In the signature for area, we use &self instead of rectangle: &Rectangle. The &self is actually short for self: &Self. Within an impl block, the type Self is an alias for the type that the impl block is for. Methods must have a parameter named self of type Self for their first parameter, so Rust lets you abbreviate this with only the name self in the first parameter spot. Note that we still need to use the & in front of the self shorthand to indicate that this method borrows the Self instance, just as we did in rectangle: &Rectangle. Methods can take ownership of self, borrow self immutably, as we’ve done here, or borrow self mutably, just as they can any other parameter.

    We chose &self here for the same reason we used &Rectangle in the function version: we don’t want to take ownership, and we just want to read the data in the struct, not write to it. If we wanted to change the instance that we’ve called the method on as part of what the method does, we’d use &mut self as the first parameter. Having a method that takes ownership of the instance by using just self as the first parameter is rare; this technique is usually used when the method transforms self into something else and you want to prevent the caller from using the original instance after the transformation.
     */
    // &self <===等价于===>self: &Self ===即===>self: &Rectangle
    // self:拥有所有权(很少见)/&self:不可变借用/&mut self:可变借用
    fn area(self: &Self) -> u32 {
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
    println!("{}", rect1.area()); // print->1500

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

/*
上述功能python实现:
class Rectangle:
    def __init__(self, width, height):
        self.width = width
        self.height = height
    
    def area(self):
        return self.width * self.height
    
    def can_hold(self, other):
        return self.width > other.width and self.height > other.height

上述功能c++实现:
class Rectangle {
private:
    int width;
    int height;
public:
    Rectangle(int width_, int height_) : width(width_), height(height_) {}

    // const对应&self(不可变借用)
    int area() const { return width * height; }

    bool can_hold(Rectangle &other) const { return width > other.width && height > other.height; }
};
*/