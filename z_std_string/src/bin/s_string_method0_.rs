// std::string::String

/*
常见方法:
len / is_empty
push_str / push
remove

Mathods from Deref<Target=str>:
len / is_empty
bytes / chars
contains
find
get
split
trim / stars_with / ends_with / to_lowercase / to_uppercase
 */

fn main() {
}

#[test]
fn len_is_empty_fn() {
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
fn push_push_str_fn() {
    let mut s = String::from("foo");

    // Appends a given string slice onto the end of this String.
    // 函数签名:`pub fn push_str(&mut self, string: &str)`
    s.push_str("bar");

    assert_eq!("foobar", s);

    let mut s = String::from("abc");

    // Appends the given char to the end of this String.
    // 函数签名:pub fn push(&mut self, ch: char)
    s.push('1');
    s.push('2');
    s.push('3');

    assert_eq!("abc123", s);
}

#[test]
fn remove_fn() {
    let mut s = String::from("foo!");

    /*
    函数签名:`pub fn remove(&mut self, idx: usize) -> char`
    Removes a char from this String at a byte position and returns it.

    This is an O(n) operation, as it requires copying every element in the buffer.

    # Panics
    Panics if idx is larger than or equal to the String’s length, or if it does not lie on a char boundary.
     */
    assert_eq!(s.remove(0), 'f');
    assert_eq!(s.remove(1), 'o');
    assert_eq!(s.remove(0), 'o');
    println!("{}", s) // 运行结果:"!'
}

