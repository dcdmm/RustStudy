// Trait std::convert::From/std::convert::Into

/*
std::convert::From: Used to do value-to-value conversions while consuming the input value. It is the reciprocal of Into.
std::convert::Into: value-to-value conversion that consumes the input value. The opposite of From.

One should avoid implementing Into and implement From instead. Implementing From automatically provides one with an implementation of Into thanks to the blanket implementation in the standard library.

blanket implementation in the standard library:
```rust
// From implies Into
#[stable(feature = "rust1", since = "1.0.0")]
impl<T, U> Into<U> for T
where
    U: From<T>,
{
    /// Calls `U::from(self)`.
    ///
    /// That is, this conversion is whatever the implementation of
    /// <code>[From]&lt;T&gt; for U</code> chooses to do.
    #[inline]
    fn into(self) -> U {
        U::from(self)
    }
}
```

# Generic Implementations
    * From<T> for U implies Into<U> for T
    * From is reflexive, which means that From<T> for T is implemented
    * Into is reflexive, which means that Into<T> for T is implemented
*/
#[test]
fn t0() {
    let string = "hello".to_string();
    // impl From<&str> for String
    let other_string = String::from("hello");
    println!("{}", other_string);

    let x_r = &[1, 2, 3, 4];
    // impl<T: Clone, const N: usize> From<&[T; N]> for Vec<T>
    let vec_t = Vec::from(x_r);
    println!("{:?}", vec_t);
}

#[test]
fn t1() {
    #[derive(Debug)]
    struct Point {
        x: i32,
        y: i32,
    }

    #[derive(Debug, Clone, Copy)]
    struct Coordinate {
        lat: f64,
        lon: f64,
    }

    impl From<Coordinate> for Point {
        fn from(coord: Coordinate) -> Self {
            Point {
                x: (coord.lat * 100.0) as i32,
                y: (coord.lon * 100.0) as i32,
            }
        }
    }

    let coord = Coordinate { lat: 1.1, lon: 2.2 };
    let point: Point = coord.into();
    println!("{:?}", point); // print->Point { x: 110, y: 220 }

    let point1 = Point::from(coord);
    println!("{:?}", point1) // print->Point { x: 110, y: 220 }
}
