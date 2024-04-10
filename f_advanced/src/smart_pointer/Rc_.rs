// std::rc::Rc

/*
The type Rc<T> provides shared ownership of a value of type T, allocated in the heap. Invoking clone on Rc produces a new pointer to the same allocation in the heap. When the last Rc pointer to a given allocation is destroyed, the value stored in that allocation (often referred to as “inner value”) is also dropped.

Shared references in Rust disallow mutation by default, and Rc is no exception: you cannot generally obtain a mutable reference to something inside an Rc. If you need mutability, put a Cell or RefCell inside the Rc; 
*/

#[allow(warnings)]
use std::{ops::Deref, rc::Rc};

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
    println!("count after creating b = {}", Rc::strong_count(&  a)); // print->count after creating b = 2

    {
        let _c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a)); // print->count after creating c = 3
    }

    println!("count after c goes out of scope = {}", Rc::strong_count(&a)); // print->count after c goes out of scope = 2
}

#[allow(warnings)]
#[derive(Debug)]
struct People {
    age: i32,
}

#[allow(warnings)]
#[test]
fn t1() {
    let p = People{age: 10}; // `p`:  move occurs because `p` has type `Rc_::People`, which does not implement the `Copy` trai
    let x = Rc::new(p); // `p`: value moved here
    // error[E0382]: borrow of moved value: `p`
    // println!("{:?}", p);
}

#[allow(warnings)]
#[test]
fn t3() {
    let p =  People{age: 10};

    let x = Rc::new(p); 
    // error[E0507]: cannot move out of an `Rc`
    // let x_deref = *x;

    let x_immut = Deref::deref(&x);
    println!("{:?}", x);
    println!("{:?}", x_immut);
}