// std::vec::Vec
/*
常见方法:
len
is_empty
get
push
remove
 */

fn main() {}


#[test]
fn len_is_empty_fn() {
    let a = vec![1, 2, 3];
    assert_eq!(a.len(), 3);
    assert!(!a.is_empty());
}

#[test]
fn get_fn() {
    let v = vec![10, 40, 30];
    /*
    turns a reference to an element or subslice depending on the type of index.

    * If given a position, returns a reference to the element at that position or None if out of bounds.
    * If given a range, returns the subslice corresponding to that range, or None if out of bounds.
     */
    assert_eq!(Some(&40), v.get(1));
    assert_eq!(Some(&[10, 40][..]), v.get(0..2));
    assert_eq!(None, v.get(3));
    assert_eq!(None, v.get(0..4));
}

#[test]
fn push_fn() {
    // As with any variable, if we want to be able to change its value, we need to make it mutable using the mut keyword
    let mut vec = vec![1, 2];
    // Appends an element to the back of a collection.
    vec.push(3);
    assert_eq!(vec, [1, 2, 3]);
}

#[test]
fn remove_fn() {
    // Removes and returns the element at position index within the vector, shifting all elements after it to the left.
    let mut v = vec![1, 2, 3];
    assert_eq!(v.remove(1), 2);
    assert_eq!(v, vec![1, 3]);

    //# Panics
    // Panics if index is out of bounds.
}
