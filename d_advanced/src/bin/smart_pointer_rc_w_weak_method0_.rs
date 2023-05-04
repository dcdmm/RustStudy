// std::rc::Weak

/*
Weak is a version of Rc that holds a non-owning reference to the managed allocation.
The allocation is accessed by calling upgrade on the Weak pointer, which returns an Option<Rc<T>>.

Since a Weak reference does not count towards ownership, it will not prevent the value stored in the allocation from being dropped, and Weak itself makes no guarantees about the value still being present.
Thus it may return None when upgraded. Note however that a Weak reference does prevent the allocation itself (the backing store) from being deallocated.

A Weak pointer is useful for keeping a temporary reference to the allocation managed by Rc without preventing its inner value from being dropped.
It is also used to prevent circular references between Rc pointers, since mutual owning references would never allow either Rc to be dropped. For example, a tree could have strong Rc pointers from parent nodes to children, and Weak pointers from children back to their parents.

The typical way to obtain a Weak pointer is to call Rc::downgrade.
 */

/*
常见方法:
new
upgrade
 */

fn main() {}

#[test]
fn new_fn() {
    use std::rc::Weak;

    /*
    pub fn new() -> Weak<T>
        Constructs a new Weak<T>, without allocating any memory. Calling upgrade on the return value always gives None.
     */

    let empty: Weak<i64> = Weak::new();
    assert!(empty.upgrade().is_none());
}

#[test]
fn upgrade_fn() {
    use std::rc::Rc;

    /*
    pub fn upgrade(&self) -> Option<Rc<T>>
        Attempts to upgrade the Weak pointer to an Rc, delaying dropping of the inner value if successful.

        Returns None if the inner value has since been dropped.
     */
    let five = Rc::new(5);
    let weak_five = Rc::downgrade(&five);
    let strong_five = weak_five.upgrade();
    assert!(strong_five.is_some());

    // Destroy all strong pointers.
    drop(strong_five);
    assert!(weak_five.upgrade().is_some());
    drop(five);
    assert!(weak_five.upgrade().is_none());
}