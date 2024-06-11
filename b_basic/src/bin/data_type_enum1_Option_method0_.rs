#![allow(non_snake_case)]

// std::option::Option

/*
常见方法:
is_none / is_some
unwarp / unwarp_or / unwarp_or_else
 */

#[test]
fn is_none_is_some_test() {
    /*
    pub const fn is_none(&self) -> bool
        Returns true if the option is a None value.

    pub const fn is_some(&self) -> bool
        Returns true if the option is a Some value.
     */
    let x: Option<u32> = Some(2);
    println!("{}", x.is_none()); // false

    let x: Option<u32> = None;
    println!("{}", x.is_none()); // true
}

#[test]
fn unwarp_test() {
    let x = Some("air");
    // Returns the contained Some value or a provided default.
    println!("{}", x.unwrap()); // print->air

    let y: Option<&str> = None;
    // panicked: called `Option::unwrap()` on a `None` value
    println!("{}", y.unwrap());
}

#[test]
fn unwarp_or_test() {
    // Returns the contained Some value or a provided default.
    println!("{}", Some("car").unwrap_or("bike")); // print->car

    println!("{}", None.unwrap_or("bike")); // print->bike
}

#[test]
fn unwarp_or_else_test() {
    let k = 10;

    // Returns the contained Some value or computes it from a closure.
    println!("{}", Some(4).unwrap_or_else(|| 2 * k)); // print->4
    println!("{}", None.unwrap_or_else(|| 2 * k)); // print->20
}

fn main() {}
