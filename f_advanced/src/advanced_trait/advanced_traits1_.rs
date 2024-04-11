// Using Supertraits to Require One Trait’s Functionality Within Another Trait

use std::fmt;

// Implementing the OutlinePrint trait that requires the functionality from Display
trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        // Because we’ve specified that OutlinePrint requires the Display trait, we can use the to_string function that is automatically implemented for any type that implements Display.
        let output = self.to_string();
        let len = output.len();
        println!("{}", "*".repeat(len + 4));
        println!("*{}*", " ".repeat(len + 2));
        println!("* {} *", output);
        println!("*{}*", " ".repeat(len + 2));
        println!("{}", "*".repeat(len + 4));
    }
}

struct Point {
    x: i32,
    y: i32,
}

// 报错:error[E0277]: `Point` doesn't implement `std::fmt::Display`
// impl OutlinePrint for Point {}
//
// 解决方式如下:
// To fix this, we implement Display on Point and satisfy the constraint that OutlinePrint requires, like so:
// *****************************************************************
impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl OutlinePrint for Point {}
// *****************************************************************