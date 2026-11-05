// 字符串切片/string slice(Primitive Type str)

/*
常见方法:
len / is_empty
bytes / chars
contains
find
get
split
trim / stars_with / ends_with / to_lowercase / to_uppercase
 */

#[test]
fn len_is_empty_test() {
    // This length is in bytes, not chars or graphemes. In other words, it might not be what a human considers the length of the string.
    let len = "foo".len();
    assert_eq!(3, len);

    let s0 = "";
    // Returns true if self has a length of zero bytes.
    assert!(s0.is_empty());

    let s1 = "not empty";
    assert!(!s1.is_empty());
}

#[test]
fn bytes_test() {
    /*
    An iterator over the bytes of a string slice.

    As a string slice consists of a sequence of bytes, we can iterate through a string slice by byte. This method returns such an iterator.
     */

    let mut bytes = "bors".bytes();
    assert_eq!(Some(b'b'), bytes.next());
    assert_eq!(Some(b'o'), bytes.next());
    assert_eq!(Some(b'r'), bytes.next());
    assert_eq!(Some(b's'), bytes.next());
    assert_eq!(None, bytes.next());
}

#[test]
fn chars_test() {
    /*
    Returns an iterator over the chars of a string slice.
    
    As a string slice consists of valid UTF-8, we can iterate through a string slice by char. This method returns such an iterator.
    */

    let word = "风!a1";
    let count = word.chars().count();
    assert_eq!(4, count);

    let mut chars = word.chars();
    assert_eq!(Some('风'), chars.next());
    assert_eq!(Some('!'), chars.next());
    assert_eq!(Some('a'), chars.next());
    assert_eq!(Some('1'), chars.next());
    assert_eq!(None, chars.next());
}

#[test]
fn contains_test() {
    /*
    Returns true if the given pattern matches a sub-slice of this string slice.
    
    Returns false if it does not.
    
    The pattern can be a &str, char, a slice of chars, or a function or closure that determines if a character matches.
    */

    let bananas = "bananas";
    assert!(bananas.contains("nana"));
    assert!(!bananas.contains("apples"));
}

#[test]
fn find_test() {
    /*
    Returns the byte index of the first character of this string slice that matches the pattern.
    
    Returns None if the pattern doesn’t match.
    
    The pattern can be a &str, char, a slice of chars, or a function or closure that determines if a character matches.
    */

    let s = "Löwe 老虎 Léopard Gepardi";
    assert_eq!(s.find('L'), Some(0));  // char
    assert_eq!(s.find('é'), Some(14));
    assert_eq!(s.find("pard"), Some(17)); // &str

    // More complex patterns using point-free style and closures:
    let s = "Löwe 老虎 Léopard";
    assert_eq!(s.find(char::is_whitespace), Some(5));
    assert_eq!(s.find(char::is_lowercase), Some(1));
    assert_eq!(s.find(|c: char| c.is_whitespace() || c.is_lowercase()), Some(1));
    assert_eq!(s.find(|c: char| (c < 'o') && (c > 'a')), Some(4));
}

#[test]
fn get_test() {
    /*
    Returns a subslice of str.
    
    This is the non-panicking alternative to indexing the str. Returns None whenever equivalent indexing operation would panic.
    */

    let v = "中🗻m∈🌏";
    let v_03 = v.get(..3);
    assert_eq!(Some("中"), v_03);
    println!("{}", "中".len()); // print->3
    assert_eq!(Some("🗻"), v.get(3..7));
    println!("{}", "🗻".len()); // print->4
    assert_eq!(Some("m"), v.get(7..8));
    println!("{}", "m".len()); // print->1

    // indices not on UTF-8 sequence boundaries
    assert!(v.get(1..).is_none());
    assert!(v.get(..6).is_none());

    // out of bounds
    assert!(v.get(..42).is_none());
}

#[test]
fn split_test() {
    /*
    An iterator over substrings of this string slice, separated by characters matched by a pattern.
    
    The pattern can be a &str, char, a slice of chars, or a function or closure that determines if a character matches.
    */
    let v: Vec<&str> = "Mary had a little lamb".split(' ').collect();
    assert_eq!(v, ["Mary", "had", "a", "little", "lamb"]);

    let v: Vec<&str> = "".split('X').collect();
    assert_eq!(v, [""]);

    let v: Vec<&str> = "lionXXtigerXleopard".split('X').collect();
    assert_eq!(v, ["lion", "", "tiger", "leopard"]);

    let v: Vec<&str> = "lion::tiger::leopard".split("::").collect();
    assert_eq!(v, ["lion", "tiger", "leopard"]);

    let v: Vec<&str> = "abc1def2ghi".split(char::is_numeric).collect();
    assert_eq!(v, ["abc", "def", "ghi"]);

    let v: Vec<&str> = "lionXtigerXleopard".split(char::is_uppercase).collect();
    assert_eq!(v, ["lion", "tiger", "leopard"]);

    // If the pattern is a slice of chars, split on each occurrence of any of the characters:
    let v: Vec<&str> = "abc1defXghi".split(|c| c == '1' || c == 'X').collect();
    assert_eq!(v, ["abc", "def", "ghi"]);

    // If a string contains multiple contiguous separators, you will end up with empty strings in the output:
    let x = "||||a||b|c".to_string();
    let d: Vec<_> = x.split('|').collect();

    assert_eq!(d, &["", "", "", "", "a", "", "b", "c"]);
}

#[test]
fn trim_stars_with_ends_with_to_lowercase_to_uppercase_test() {
    let s = "\n Hello\tworld\t\n";
    assert_eq!("Hello\tworld", s.trim());

    let bananas = "bananas";
    /*
    Returns true if the given pattern matches a prefix of this string slice.

    Returns false if it does not.

    The pattern can be a &str, char, a slice of chars, or a function or closure that determines if a character matches.
     */
    assert!(bananas.starts_with("bana"));
    assert!(!bananas.starts_with("nana"));
    
    /*
    Returns true if the given pattern matches a suffix of this string slice.

    Returns false if it does not.

    The pattern can be a &str, char, a slice of chars, or a function or closure that determines if a character matches.
     */
    assert!(bananas.ends_with("anas"));
    assert!(!bananas.ends_with("nana"));

    let s = "HELLO";
    assert_eq!("hello", s.to_lowercase());

    let s = "hello";
    assert_eq!("HELLO", s.to_uppercase());
}

fn main() {}
