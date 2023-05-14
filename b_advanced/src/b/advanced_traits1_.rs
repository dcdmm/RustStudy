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