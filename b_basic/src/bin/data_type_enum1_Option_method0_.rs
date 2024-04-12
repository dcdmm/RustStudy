#![allow(non_snake_case)]

// std::option::Option

/*
常见方法:
is_none / is_some
unwarp
unwarp_or
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
fn unwarp_or_test() {
    let x = Some("air");
    // Returns the contained Some value or a provided default.
    println!("{}", x.unwrap());

    let y: Option<&str> = None;
    // panicked: called `Option::unwrap()` on a `None` value
    println!("{}", y.unwrap());
}

fn main() {}
