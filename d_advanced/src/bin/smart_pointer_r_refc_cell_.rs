// std::cell::RefCell
// Having Multiple Owners of Mutable Data by Combining Rc<T> and RefCell<T>

/*
A common way to use RefCell<T> is in combination with Rc<T>.
Recall that Rc<T> lets you have multiple owners of some data, but it only gives immutable access to that data.
If you have an Rc<T> that holds a RefCell<T>, you can get a value that can have multiple owners and that you can mutate!
 */
#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("a after = {:?}", a);
    println!("b after = {:?}", b);
    println!("c after = {:?}", c);
}