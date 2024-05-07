// std::collections::HashSet

/*
常见方法:
len
is_empty

clear

get

insert

remove
 */

use std::collections::HashSet;

#[test]
fn len_is_empty_test() {
    let mut v = HashSet::new();
    assert_eq!(v.len(), 0);
    assert!(v.is_empty());
    v.insert(1);
    assert_eq!(v.len(), 1);
    assert!(!v.is_empty());
}

#[test]
fn clear_test() {
    let mut v = HashSet::from([1, 2, 3]);
    v.clear();
    assert!(v.is_empty());
}

#[test]
fn get() {
    let set = HashSet::from([1, 2, 3]);

    // Returns a reference to the value in the set, if any, that is equal to the given value.
    assert_eq!(set.get(&2), Some(&2));
    assert_eq!(set.get(&4), None);
}

#[test]
fn insert_test() {
    let mut set = HashSet::new();

    /*
    Adds a value to the set.

    Returns whether the value was newly inserted. That is:
        * If the set did not previously contain this value, true is returned.
        * If the set already contained this value, false is returned, and the set is not modified: original value is not replaced, and the value passed as argument is dropped.
     */
    assert_eq!(set.insert(2), true);
    assert_eq!(set.insert(2), false);
    assert_eq!(set.len(), 1);
}

#[test]
fn remove_test() {
    let mut set = HashSet::from([1, 2, 3, 4, 5]);

    // Removes a value from the set. Returns whether the value was present in the set.
    assert_eq!(set.remove(&2), true);
    println!("{:?}", set); // print->{4, 3, 5, 1}
    assert_eq!(set.remove(&2), false);
    println!("{:?}", set); // print->{4, 3, 5, 1}
}
