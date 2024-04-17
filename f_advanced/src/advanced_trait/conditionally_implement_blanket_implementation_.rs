// Conditionally Implement(blanket_implementation)

use std::fmt::Display;

trait ToString {}
/*
We can also conditionally implement a trait for any type that implements another trait. Implementations of a trait on any type that satisfies the trait bounds are called blanket implementations and are extensively used in the Rust standard library. For example, the standard library implements the ToString trait on any type that implements the Display trait.
The impl block in the standard library looks similar to this code:
 */
impl<T: Display> ToString for T {
    // --snip--
}

#[test]
fn t0() {
    /*
    Because the standard library has this blanket implementation, we can call the to_string method defined by the ToString trait on any type that implements the Display trait. For example, we can turn integers into their corresponding String values like this because integers implement Display:
     */
    let s = 3.to_string();
    println!("{}", s);
}

#[test]
fn t1() {
    trait Printable {
        fn print(&self);
    }

    impl<T: Display> Printable for T {
        fn print(&self) {
            println!("{}", self);
        }
    }

    let my_string = "Hello, world!".to_string();
    my_string.print(); // String实现了Display特质,所以也实现了Printable特质
    let my_number = 42;
    my_number.print(); // i32实现了Display特质,所以也实现了Printable特质
}
