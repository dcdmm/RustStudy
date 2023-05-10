// trait(Using Trait Bounds to Conditionally Implement Methods)

use std::fmt::Display;

#[allow(dead_code)]
struct Pair<T> {
    x: T,
    y: T,
}

// 关联函数
#[allow(dead_code)]
impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

// 只有T同时实现了Display + PartialOrd trait结构体Pair才拥有cmp_display方法
#[allow(dead_code)]
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
We can also conditionally implement a trait for any type that implements another trait.
Implementations of a trait on any type that satisfies the trait bounds are called blanket implementations and are extensively used in the Rust standard library.
For example, the standard library implements the ToString trait on any type that implements the Display trait.
The impl block in the standard library looks similar to this code:
 */
impl<T: Display> ToString for T {
    // --snip--
}

fn main() {
    /*
    Because the standard library has this blanket implementation, we can call the to_string method defined by the ToString trait on any type that implements the Display trait.
    For example, we can turn integers into their corresponding String values like this because integers implement Display:
     */
    let s = 3.to_string();
    println!("{}", {s});
}
