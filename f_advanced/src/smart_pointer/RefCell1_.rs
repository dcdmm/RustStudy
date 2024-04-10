// std::cell::RefCell

// Having Multiple Owners of Mutable Data by Combining Rc<T> and RefCell<T>

/*
A common way to use RefCell<T> is in combination with Rc<T>. Recall that Rc<T> lets you have multiple owners of some data, but it only gives immutable access to that data. If you have an Rc<T> that holds a RefCell<T>, you can get a value that can have multiple owners and that you can mutate!
 */
#[allow(warnings)]
#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

use std::cell::RefCell;
use std::rc::Rc;
#[allow(warnings)]
use List::{Cons, Nil};

#[test]
fn t0() {
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));

    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    *value.borrow_mut() += 10;

    println!("{:?}", a); // print->Cons(RefCell { value: 15 }, Nil)
    println!("{:?}", b); // print->Cons(RefCell { value: 3 }, Cons(RefCell { value: 15 }, Nil))
    println!("{:?}", c); // print->Cons(RefCell { value: 4 }, Cons(RefCell { value: 15 }, Nil))
}

#[allow(warnings)]
#[test]
fn t1() {
    use std::cell::RefCell;

    #[allow(warnings)]
    #[derive(Debug)]
    struct People {
        age: i32,
    }

    let x = People{age: 10};
    let c = RefCell::new(x);
    // error[E0382]: borrow of moved value: `x`
    // println!("{:?}", x);
}

#[allow(warnings)]
#[test]
fn t2() {
    use std::cell::RefCell;

    let c = RefCell::new(5);
    // error[E0614]: type `RefCell<{integer}>` cannot be dereferenced
    // let zz = *c;
}
