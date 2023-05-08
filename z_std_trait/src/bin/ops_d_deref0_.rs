// Trait std::ops::Deref

/*
Used for immutable dereferencing operations, like *v.

In addition to being used for explicit dereferencing operations with the (unary) * operator in immutable contexts, Deref is also used implicitly by the compiler in many circumstances. This mechanism is called ‘Deref coercion’. In mutable contexts, DerefMut is used.

Implementing Deref for smart pointers makes accessing the data behind them convenient, which is why they implement Deref. On the other hand, the rules regarding Deref and DerefMut were designed specifically to accommodate smart pointers. Because of this, Deref should only be implemented for smart pointers to avoid confusion.

For similar reasons, this trait should never fail. Failure during dereferencing can be extremely confusing when Deref is invoked implicitly.

pub trait Deref {
    type Target: ?Sized;

    // Required method
    fn deref(&self) -> &Self::Target;
}
 */

use std::ops::Deref;

struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    // The type Target = T; syntax defines an associated type for the Deref trait to use.
    type Target = T;

    fn deref(&self) -> &Self::Target {
        // We fill in the body of the deref method with &self.0 so deref returns a reference to the value we want to access with the * operator;
        &self.0
    }
}

fn main() {
    let x = 5;
    let y = &x;
    assert_eq!(5, x);
    // Using the dereference operator to follow a reference to an i32 value
    assert_eq!(5, *y);
    // 报错:error[E0277]: can't compare `{integer}` with `&{integer}`
    // assert_eq!(5, y);


    let x0 = 5;
    let y0 = Box::new(x);
    assert_eq!(5, x0);
    // Using the dereference operator on a Box<i32>
    assert_eq!(5, *y0);
    // 报错:error[E0277]: can't compare `{integer}` with `Box<{integer}>`
    // assert_eq!(5, y0);


    let x1 = 5;
    let y1 = MyBox::new(x);
    assert_eq!(5, x1);

    assert_eq!(5, *(y1.deref()));
    assert_eq!(5, *y1); // 与上等价
}