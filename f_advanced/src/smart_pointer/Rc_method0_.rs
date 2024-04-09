// std::rc::Rc

/*
常见方法:
new
clone / strong_count
downgrade / weak_count
get_mut
 */

#[test]
fn new_test() {
    use std::rc::Rc;

    /*
    pub fn new(value: T) -> Rc<T>
        Constructs a new Rc<T>.
     */
    let _five = Rc::new(5);
}

#[test]
fn clone_strong_count_test() {
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
fn downgrade_weak_count_test() {
    use std::rc::Rc;

    /*
    pub fn downgrade(this: &Rc<T>) -> Weak<T>
        Creates a new Weak pointer to this allocation.


    pub fn weak_count(this: &Rc<T>) -> usize
        Gets the number of Weak pointers to this allocation.
     */
    let five = Rc::new(5);

    println!("{}", Rc::weak_count(&five)); // print->0
    println!("{}", Rc::strong_count(&five)); // print->1
    let _weak_five = Rc::downgrade(&five);
    println!("{}", Rc::weak_count(&five)); // print->1
    println!("{}", Rc::strong_count(&five)); // print->1
}

#[allow(warnings)]
#[test]
fn get_mut_test() {
    use std::rc::Rc;

    let mut x = Rc::new(3);
    println!("{}", Rc::strong_count(&x)); // print->1

    /*
    Returns a mutable reference into the given Rc, if there are no other Rc or Weak pointers to the same allocation.

    Returns None otherwise, because it is not safe to mutate a shared value.
     */
    let x_mut0 = Rc::get_mut(&mut x);
    println!("{:?}", x_mut0); // print->Some(3)
    let x_mut0 = x_mut0.unwrap();
    *x_mut0 = 100;
    println!("{}", x); // print->100

    let y = Rc::clone(&x);
    println!("{}", Rc::strong_count(&x)); // print->2
    let x_mut1 = Rc::get_mut(&mut x);
    println!("{:?}", x_mut1); // print->None
}
