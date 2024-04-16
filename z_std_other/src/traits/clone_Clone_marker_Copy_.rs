// Trait std::clone::Clone/std::marker::Copy

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

#[derive(Clone, Copy, Debug)]
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
