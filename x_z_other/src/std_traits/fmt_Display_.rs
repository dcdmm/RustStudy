// Trait std::fmt::Display

/*
Format trait for an empty format, {}.

Implementing this trait for a type will automatically implement the ToString trait for the type, allowing the usage of the .to_string() method. Prefer implementing the Display trait for a type, rather than ToString.

Display is similar to Debug, but Display is for user-facing output, and so cannot be derived.
*/

// # Required Methods
/*
fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>
    Formats the value using the given formatter.
*/

use std::fmt;

#[test]
fn t0() {
    struct Point {
        x: i32,
        y: i32,
    }

    // 必须手动实现
    impl fmt::Display for Point {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "<<<{}, {}>>>", self.x, self.y) // 将内容写入f,语法与format!相同
        }
    }

    let origin = Point { x: 0, y: 0 };
    println!("{}", origin); // print-><<<0, 0>>>
    assert_eq!(origin.to_string(), "<<<0, 0>>>"); // 实现Display后自动获得to_string方法
}
