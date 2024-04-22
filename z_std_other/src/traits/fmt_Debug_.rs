// Trait std::fmt::Debug

/*
? formatting.

When used with the alternate format specifier #?, the output is pretty-printed.

This trait can be used with #[derive] if all fields implement Debug.
*/

// # Required Methods
/*
fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error>
    Formats the value using the given formatter.
*/
#[test]
fn t0() {
    #[derive(Debug)]
    struct Point {
        x: i32,
        y: i32,
    }

    let origin = Point { x: 0, y: 0 };
    println!("{:?}", origin);
    println!("{:#?}", origin)
}

#[test]
fn t1() {
    use std::fmt;

    struct Point {
        x: i32,
        y: i32,
    }

    impl fmt::Debug for Point {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("Point")
                .field("x", &self.x)
                .field("y", &self.y)
                .finish()
        }
    }

    let origin = Point { x: 0, y: 0 };
    println!("{:?}", origin);
    println!("{:#?}", origin);
}
