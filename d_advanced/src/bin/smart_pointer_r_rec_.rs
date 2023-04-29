// Rc

use std::rc::Rc;

enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    /*
    pub fn strong_count(this: &Rc<T>) -> usize
        Gets the number of strong (Rc) pointers to this allocation.
     */
    println!("count after creating a = {}", Rc::strong_count(&a)); // 1
    /*
    fn clone(&self) -> Rc<T>
        Makes a clone of the Rc pointer.

        This creates another pointer to the same allocation, increasing the strong reference count.
     */
    let _b = Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a)); // 2
    {
        let _c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a)); // 3
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a)); // 2
}