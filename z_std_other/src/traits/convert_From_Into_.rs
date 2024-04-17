// Trait std::convert::From/std::convert::Into

// std::convert::From: Used to do value-to-value conversions while consuming the input value. It is the reciprocal of Into.
// std::convert::Into: value-to-value conversion that consumes the input value. The opposite of From.

/*
One should avoid implementing Into and implement From instead. Implementing From automatically provides one with an implementation of Into thanks to the blanket implementation in the standard library.

the blanket implementation code in the standard library is as follows:
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

// # 常见实现
/*
impl From<&str> for String
impl From<&String> for String
impl From<&mut str> for String
impl From<char> for String
impl From<Box<str>> for String

impl From<&str> for Vec<u8>
impl From<String> for Vec<u8>
impl<T> From<&[T]> for Vec<T> where T: Clone,
impl<T> From<&mut [T]> for Vec<T> where T: Clone,
impl<T, const N: usize> From<&[T; N]> for Vec<T> where T: Clone,
impl<T, const N: usize> From<&mut [T; N]> for Vec<T> where T: Clone,
impl<T, const N: usize> From<[T; N]> for Vec<T>
impl<T, A> From<Box<[T], A>> for Vec<T, A> where A: Allocator,
impl<T, A> From<VecDeque<T, A>> for Vec<T, A> where A: Allocator,

impl<T, A> From<Vec<T, A>> for VecDeque<T, A> where A: Allocator,
impl<T, const N: usize> From<[T; N]> for VecDeque<T>

impl<K, V, const N: usize> From<[(K, V); N]> for HashMap<K, V, RandomState> where K: Eq + Hash,

impl<T, const N: usize> From<[T; N]> for HashSet<T, RandomState> where T: Eq + Hash,
*/

#[test]
fn t0() {
    // impl From<&str> for String
    let string_f0 = String::from("hello");
    println!("{}", string_f0);
    let c = 'a';
    // impl From<char> for String
    let string_f1 = String::from(c);

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
