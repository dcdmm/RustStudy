// 泛型(In Method Definitions)

#[allow(dead_code)]
struct Point<T> {
    x: T,
    y: T,
}

/*
 Note that we have to declare T just after impl so we can use T to specify that we’re implementing methods on the type Point<T>.
 By declaring T as a generic type after impl, Rust can identify that the type in the angle brackets in Point is a generic type rather than a concrete type.
 We could have chosen a different name for this generic parameter than the generic parameter declared in the struct definition, but using the same name is conventional.
 Methods written within an impl that declares the generic type will be defined on any instance of the type, no matter what concrete type ends up substituting for the generic type.
 */
#[allow(dead_code)]
impl<T> Point<T> {
    fn return_xy(&self) -> Point<&T> {
        Point{x: &self.x, y: &self.x}
    }
}

/*
We can also specify constraints on generic types when defining methods on the type.
We could, for example, implement methods only on Point<f32> instances rather than on Point<T> instances with any generic type.
 Here we use the concrete type f32, meaning we don’t declare any types after impl.
 */
#[allow(dead_code)]
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

#[allow(dead_code)]
struct Point1<X1, Y1> {
    x: X1,
    y: Y1,
}

// Generic type parameters in a struct definition aren’t always the same as those you use in that same struct’s method signatures.
#[allow(dead_code)]
impl <X1, Y1> Point1<X1, Y1>{
    fn mixup<X2, Y2>(self, other: Point1<X2, Y2>) -> Point1<X1, Y2> {
        Point1 { x: self.x, y: other.y }
    }
}

fn main() {
    let p = Point { x: 5.4, y: 10.5 };
    println!("p.x = {}", p.return_xy().x);
    println!("p.y = {}", p.return_xy().y);
    println!("{}", p.distance_from_origin());


    let p1 = Point1{x: 5, y:10.4};
    let p2 = Point1{x: "hello", y: 'c'};
    let p3 = p1.mixup(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}
