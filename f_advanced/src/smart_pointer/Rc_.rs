// std::rc::Rc

// A single-threaded reference-counting pointer. ‘Rc’ stands for ‘Reference Counted’.

use std::rc::Rc;

#[allow(warnings)]
enum List {
    Cons(i32, Rc<List>),
    Nil,
}

#[allow(warnings)]
use List::{Cons, Nil};

#[test]
fn t0() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a)); // print->count after creating a = 1

    let _b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a)); // print->count after creating b = 2

    {
        let _c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a)); // print->count after creating c = 3
    }

    println!("count after c goes out of scope = {}", Rc::strong_count(&a)); // print->count after c goes out of scope = 2
}
