// std::string::String

/*
方法学习:
as_str / as_mut_str
 */

#[test]
fn as_str_as_mut_str_fn() {
    let s = String::from("foo");
    // Extracts a string slice containing the entire String.
    assert_eq!("foo", s.as_str());

    let mut s = String::from("foobar");
    // Converts a String into a mutable string slice.
    let s_mut_str = s.as_mut_str();

    s_mut_str.make_ascii_uppercase();

    assert_eq!("FOOBAR", s_mut_str);
}