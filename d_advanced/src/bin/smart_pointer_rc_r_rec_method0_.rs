// std::rc::Rc
/*
常见方法:
new
clone / strong_count
downgrade / weak_count
 */

fn main() {}

#[test]
fn new_fn() {
    use std::rc::Rc;

    /*
    pub fn new(value: T) -> Rc<T>
        Constructs a new Rc<T>.
     */
    let _five = Rc::new(5);
}

#[test]
fn clone_strong_count_fn() {
    use std::rc::Rc;

    /*
     clone(&self) -> Rc<T>
        Makes a clone of the Rc pointer.

        This creates another pointer to the same allocation, increasing the strong reference count.


    pub fn strong_count(this: &Rc<T>) -> usize
        Gets the number of strong (Rc) pointers to this allocation.
     */
    let five = Rc::new(5);
    let _also_five = Rc::clone(&five);

    assert_eq!(2, Rc::strong_count(&five));
}

#[test]
fn downgrade_weak_count_fn() {
    use std::rc::Rc;

    /*
    pub fn downgrade(this: &Rc<T>) -> Weak<T>
        Creates a new Weak pointer to this allocation.


    pub fn weak_count(this: &Rc<T>) -> usize
        Gets the number of Weak pointers to this allocation.
     */
    let five = Rc::new(5);

    let _weak_five = Rc::downgrade(&five);
    assert_eq!(1, Rc::weak_count(&five));
}