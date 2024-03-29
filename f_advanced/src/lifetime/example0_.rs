// Generic Type Parameters, Trait Bounds, and Lifetimes Together example

use std::fmt::Display;

#[allow(warnings)]
fn longest_with_an_announcement<'a, T>(x: &'a str, y: &'a str, ann: T) -> &'a str
where
    T: Display, // trait Bounds
{
    println!("Announcement! {}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

#[test]
fn t0() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest_with_an_announcement(string1.as_str(), string2, "hello rust!");
    println!("The longest string is {}", result);
}
