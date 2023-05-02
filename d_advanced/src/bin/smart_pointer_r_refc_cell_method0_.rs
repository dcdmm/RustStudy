// std::cell::RefCell
/*
常见方法:
new
borrow / borrow_mut
 */

use std::cell::RefCell;

fn main() {}

#[test]
fn new_fn() {
    /*
    pub const fn new(value: T) -> RefCell<T>
        Creates a new RefCell containing value.
     */
    let _five = RefCell::new(5);
}

#[test]
fn borrow_borrow_mut_fn0() {
    /*
    pub fn borrow(&self) -> Ref<'_, T>
        Immutably borrows the wrapped value.

        The borrow lasts until the returned Ref exits scope. Multiple immutable borrows can be taken out at the same time.

    Panics
        Panics if the value is currently mutably borrowed. For a non-panicking variant, use try_borrow.


    pub fn borrow_mut(&self) -> RefMut<'_, T>
        Mutably borrows the wrapped value.

        The borrow lasts until the returned RefMut or all RefMuts derived from it exit scope. The value cannot be borrowed while this borrow is active.

    Panics
        Panics if the value is currently borrowed. For a non-panicking variant, use try_borrow_mut.
     */

    let c = RefCell::new(5);

    let _borrowed_five = c.borrow();
    let _borrowed_five2 = c.borrow();


    let c1 = RefCell::new("hello".to_owned());

    *c1.borrow_mut() = "bonjour".to_owned();

    assert_eq!(&*c1.borrow(), "bonjour");
}

#[test]
fn borrow_borrow_mut_fn1() {
    // An example of panic:
    let c = RefCell::new(5);

    let _m = c.borrow_mut();
    let _b = c.borrow(); // this causes a panic
}