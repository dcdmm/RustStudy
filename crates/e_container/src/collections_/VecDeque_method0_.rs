// std::collections::VecDeque

/*
常见方法:
len
is_empty

clear

push_back / push_front
insert

pop_back / pop_front
remove
*/
use std::collections::VecDeque;

#[test]
fn len_is_empty_test() {
    let mut deque = VecDeque::new();
    assert_eq!(deque.len(), 0);
    assert!(deque.is_empty());
    deque.push_back(1);
    assert_eq!(deque.len(), 1);
    assert!(!deque.is_empty());
}

#[test]
fn clear_test() {
    let mut vd = VecDeque::from([-1, -2, -3, 1, 2, 3]);

    // Clears the deque, removing all values.
    vd.clear();
    println!("{:?}", vd);
    println!("{:?}", vd.len());
    println!("{:?}", vd.is_empty());
}

#[test]
fn push_backa_push_front_test() {
    let mut buf = VecDeque::new();

    // Appends an element to the back of the deque.
    buf.push_back(1);
    buf.push_back(3);
    println!("{:?}", buf);

    // Prepends an element to the deque.
    buf.push_front(-1);
    buf.push_front(-3);
    println!("{:?}", buf);
}

#[test]
fn insert_test() {
    let mut vd = VecDeque::from([-1, -2, -3, 1, 2, 3]);

    /*
    Inserts an element at index within the deque, shifting all elements with indices greater than or equal to index towards the back.

    Element at index 0 is the front of the queue.

    # Panics
    Panics if index is greater than deque’s length
     */
    vd.insert(3, 0);
    println!("{:?}", vd); // print->[-1, -2, -3, 0, 1, 2, 3]
}

#[test]
fn pop_back_pop_front() {
    let mut vd = VecDeque::from([-1, -2, -3, 1, 2, 3]);

    // Removes the last element from the deque and returns it, or None if it is empty.
    vd.pop_back();
    println!("{:?}", vd);

    // Removes the first element and returns it, or None if the deque is empty.
    vd.pop_front();
    println!("{:?}", vd);
}

#[test]
fn remove_test() {
    let mut vd = VecDeque::from([-1, -2, -3, 1, 2, 3]);

    // Removes and returns the element at index from the deque. Whichever end is closer to the removal point will be moved to make room, and all the affected elements will be moved to new positions. Returns None if index is out of bounds.
    println!("{:?}", vd.remove(1)); // print->Some(-2)
    println!("{:?}", vd); // print->[-1, -3, 1, 2, 3]
}
