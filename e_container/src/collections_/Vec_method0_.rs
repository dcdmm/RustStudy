// std::vec::Vec

/*
常见方法:
Methods from Deref<Target = [T]>: get / split / join / sort

len ===> C++:size/Python:__len__
is_empty ====> C++:empty

clear ===> C++:clear/Python:clear

push ===> C++:push_back/Python:append
insert ===> C++:insert/Python:insert
pop ===> C++:pop_back/Python:pop
remove ===> C++:erase/Python:pop(n)
    * 删除所有出现的value===>C++:std::remove+erase
    * 删除第一次出现的value===>Python:remove
reverse ===> C++:std::reverse/Python:reverse
 */

#[test]
fn len_is_empty_test() {
    let a = vec![1, 2, 3];
    assert_eq!(a.len(), 3);
    // Returns true if the vector contains no elements.
    assert!(!a.is_empty());
}

#[test]
fn clear_test() {
    let mut v = vec![1, 2, 3];

    v.clear();

    assert!(v.is_empty());
}

#[test]
fn push_test() {
    // As with any variable, if we want to be able to change its value, we need to make it mutable using the mut keyword
    let mut vec = vec![1, 2];

    // Appends an element to the back of a collection.
    vec.push(3);
    println!("{:?}", vec);

    let mut vec_str = Vec::new();
    let s = String::from("hello");
    vec_str.push(s); // `s`: value moved here
                     // error[E0382]: borrow of moved value: `s`
                     // println!("{}", s);
}

#[test]
fn insert_test() {
    let mut vec = vec![1, 2, 3];

    /*
    Inserts an element at position index within the vector, shifting all elements after it to the right.

    # Panics
    Panics if index > len.
     */
    vec.insert(1, 4);
    assert_eq!(vec, [1, 4, 2, 3]);
    vec.insert(4, 5);
    assert_eq!(vec, [1, 4, 2, 3, 5]);
}

#[test]
fn pop_test() {
    let mut vec = vec![1, 2, 3];
    // Removes the last element from a vector and returns it, or None if it is empty.
    assert_eq!(vec.pop(), Some(3));
    assert_eq!(vec, [1, 2]);
}
#[test]
fn remove_test() {
    /*
    Removes and returns the element at position index within the vector, shifting all elements after it to the left.

    # Panics
    Panics if index is out of bounds.
     */
    let mut v = vec![1, 2, 3];
    println!("{:?}", v.remove(1)); // print->2
    println!("{:?}", v); // print->[1, 3]
}

#[test]
fn reverse_test() {
    let mut vec = vec![1, 2, 3, 4, 5];
    println!("Original vector: {:?}", vec);

    // Reverses the order of elements in the slice, in place.
    vec.reverse();
    println!("Reversed vector: {:?}", vec);
}