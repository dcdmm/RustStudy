// Trait std::clone::Clone/std::marker::Copy

/*
Copies happen implicitly, for example as part of an assignment y = x. The behavior of Copy is not overloadable; it is always a simple bit-wise copy.

Cloning is an explicit action, x.clone(). The implementation of Clone can provide any type-specific behavior necessary to duplicate values safely. For example, the implementation of Clone for String needs to copy the pointed-to string buffer in the heap. A simple bitwise copy of String values would merely copy the pointer, leading to a double free down the line. For this reason, String is Clone but not Copy.

Clone is a supertrait of Copy, so everything which is Copy must also implement Clone. If a type is Copy then its Clone implementation only needs to return *self (see the example above).
*/

// # 常见实现
/*
Copy/Clone: bool/char/f32/f64/i8/i16/i32/i64/i128/u8/u16/u32/u64/u128/()/isize/usize/Error
Copy/Clone:
    Option<T>/[T; N]
        T: Copy,/T: Clone,
    *const T/ *mut T/&T/
        T: ?Sized,
    Result<T, E>
        T: Copy,/T: Clone,
        E: Copy,/E: Clone,
Clone:
    String
    Vec<T, A>
        T: Clone,
        A: Allocator + Clone,
    VecDeque<T, A>
        T: Clone,
        A: Allocator + Clone,
    HashMap<K, V, S>
        K: Clone,
        V: Clone,
        S: Clone,
    HashSet<T, S>
        T: Clone,
        S: Clone,
Clone: Thread
Clone:
    Box<T, A>
        T: Clone,
        A: Allocator + Clone,
    Box<T, A>
        T: Clone,
        A: Allocator + Clone,
    Rc<T, A>
        A: Allocator + Clone,
        T: ?Sized,
    std::rc::Weak<T, A>
        A: Allocator + Clone,
        T: ?Sized,
    RefCell<T>
        T: Clone,
*/

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T: Copy> Copy for Point<T> {}

impl<T: Clone> Clone for Point<T> {
    fn clone(&self) -> Self {
        Point {
            x: self.x.clone(),
            y: self.y.clone(),
        }
    }
}

#[derive(Clone, Copy, Debug)] // 所有字段都实现Clone、Copy特质时
struct Point1<T: Clone + Copy> {
    x: T,
    y: T,
}

#[test]
fn t0() {
    let p = Point { x: 1, y: 2 };
    let p_c = p.clone();
    println!("{:?}", p_c);

    let p1 = Point1 { x: 1, y: 2 };
    let p1_c = p1.clone();
    println!("{:?}", p1_c);
}