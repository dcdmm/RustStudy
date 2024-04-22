// std::string::String

/*
方法学习:
as_str
as_mut_str
as_mut_vec
 */

#[test]
fn as_str_as_mut_str_test() {
    let s = String::from("foo");
    // Extracts a string slice containing the entire String.
    let s_str = s.as_str();

    let mut s = String::from("foobar");
    // Converts a String into a mutable string slice.
    let s_mut_str = s.as_mut_str();
}

#[test]
fn as_mut_vec_test() {
    let mut s = String::from("hello");

    unsafe {
        /*
        Returns a mutable reference to the contents of this String.

        # Safety
        This function is unsafe because the returned &mut Vec allows writing bytes which are not valid UTF-8. If this constraint is violated, using the original String after dropping the &mut Vec may violate memory safety, as the rest of the standard library assumes that Strings are valid UTF-8.
         */
        let vec = s.as_mut_vec();
        println!("{:?}", vec);
        vec.reverse();
    }
    println!("{}", s);
}
