// std::string::String

/*
常见方法:
Mathods from Deref<Target=str>:
bytes / chars
contains
find
get
split
trim / stars_with / ends_with / to_lowercase / to_uppercase


len / is_empty

clear

push
push_str
insert

pop
remove

reserve
 */

#[test]
fn len_is_empty_test() {
    let a = String::from("foo");
    // This length is in bytes, not chars or graphemes. In other words, it might not be what a human considers the length of the string.
    assert_eq!(a.len(), 3);

    let mut v = String::new();
    // Returns true if this String has a length of zero, and false otherwise.
    assert!(v.is_empty());
    v.push('a');
    assert!(!v.is_empty());
}

#[test]
fn clear_test() {
    let mut s = String::from("foo");

    s.clear();
    assert!(s.is_empty());
    assert_eq!(0, s.len())
}

#[test]
fn push_push_str_test() {
    let mut s = String::from("foo");

    // Appends a given string slice onto the end of this String.
    s.push_str("bar");
    println!("{}", s);

    let mut s = String::from("abc");

    // Appends the given char to the end of this String.
    s.push('1');
    s.push('2');
    s.push('3');
    println!("{}", s);
}

#[test]
fn pop_test() {
    let mut s = String::from("abč");

    /*
    moves the last character from the string buffer and returns it.

    Returns None if this String is empty.
     */
    assert_eq!(s.pop(), Some('č'));
    println!("{}", s);
    assert_eq!(s.pop(), Some('b'));
    println!("{}", s);
    assert_eq!(s.pop(), Some('a'));
    println!("{}", s);
    assert_eq!(s.pop(), None);
}

#[test]
fn remove_test() {
    let mut s = String::from("foo!");

    /*
    Removes a char from this String at a byte position and returns it.

    This is an O(n) operation, as it requires copying every element in the buffer.

    # Panics
    Panics if idx is larger than or equal to the String’s length, or if it does not lie on a char boundary.
     */
    assert_eq!(s.remove(0), 'f');
    println!("{}", s); // print->oo!
    assert_eq!(s.remove(1), 'o');
    assert_eq!(s.remove(0), 'o');
    println!("{}", s); // print->!
}