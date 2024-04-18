// Trait std::ops::Add

// addition operator +.

// # Required Methods
/*
fn add(self, rhs: Rhs) -> Self::Output
    Performs the + operation.
*/

#[test]
fn t0() {
    use std::ops::Add;

    #[derive(Debug, Copy, Clone, PartialEq)]
    struct Point {
        x: i32,
        y: i32,
    }

    impl Add for Point {
        type Output = Self;

        fn add(self, other: Self) -> Self {
            Self {
                x: self.x + other.x,
                y: self.y + other.y,
            }
        }
    }

    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 }
    );
}

#[test]
fn t1() {
    use std::ops::Add;

    #[derive(Debug, Copy, Clone, PartialEq)]
    struct Point<T> {
        x: T,
        y: T,
    }

    // Notice that the implementation uses the associated type `Output`.
    impl<T: Add<Output = T>> Add for Point<T> {
        type Output = Self;

        fn add(self, other: Self) -> Self::Output {
            Self {
                x: self.x + other.x,
                y: self.y + other.y,
            }
        }
    }

    assert_eq!(
        Point { x: 1, y: 0 } + Point { x: 2, y: 3 },
        Point { x: 3, y: 3 }
    );
}
