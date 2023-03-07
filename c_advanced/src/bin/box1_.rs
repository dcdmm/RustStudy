// Box

/*
Enabling Recursive Types with Boxes
A value of recursive type can have another value of the same type as part of itself. Recursive types pose an issue because at compile time Rust needs to know how much space a type takes up. However, the nesting of values of recursive types could theoretically continue infinitely, so Rust can’t know how much space the value needs. Because boxes have a known size, we can enable recursive types by inserting a box in the recursive type definition.
 */

// enum List {
//     Cons(i32, List),
//     Nil,
// }
// 程序运行结果:
// error[E0072]: recursive type `List` has infinite size
//  --> c_advanced\src\bin\box1_.rs:8:1
//   |
// 8 | enum List {
//   | ^^^^^^^^^ recursive type has infinite size
// 9 |     Cons(i32, List),
//   |               ---- recursive without indirection
//   |
// help: insert some indirection (e.g., a `Box`, `Rc`, or `&`) to make `List` representable
//   |
// 9 |     Cons(i32, Box<List>),
//   |               ++++    +

// use crate::List::{Cons, Nil};


// fn main() {
//     let list = Cons(1, Cons(2, Cons(3, Nil)));
// }

// The Cons variant needs the size of an i32 plus the space to store the box’s pointer data. The Nil variant stores no values, so it needs less space than the Cons variant.
enum List {
    Cons(i32, Box<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    let _list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
}
