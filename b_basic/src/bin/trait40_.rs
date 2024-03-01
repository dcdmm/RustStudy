// trait(Using Trait Bounds to Conditionally Implement Methods)

use std::fmt::Display;

#[allow(warnings)]
struct Pair<T> {
    x: T,
    y: T,
}

#[allow(warnings)]
impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

// Pair<T> only implements the cmp_display method if its inner type T implements the PartialOrd trait that enables comparison and the Display trait that enables printing.
#[allow(warnings)]
impl<T: Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("The largest member is x = {}", self.x);
        } else {
            println!("The largest member is y = {}", self.y);
        }
    }
}

trait ToString {}
/*
We can also conditionally implement a trait for any type that implements another trait. Implementations of a trait on any type that satisfies the trait bounds are called blanket implementations and are extensively used in the Rust standard library. For example, the standard library implements the ToString trait on any type that implements the Display trait.
The impl block in the standard library looks similar to this code:
 */
impl<T: Display> ToString for T {
    // --snip--
}

fn main() {
    let ps = Pair{x:1, y:2};
    ps.cmp_display(); // i32实现了Display + PartialOrd特质

    /*
    Because the standard library has this blanket implementation, we can call the to_string method defined by the ToString trait on any type that implements the Display trait. For example, we can turn integers into their corresponding String values like this because integers implement Display:
     */
    let s = 3.to_string();
    println!("{}", s);
}
