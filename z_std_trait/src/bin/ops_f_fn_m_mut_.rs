// Trait std::ops::FnMut
/*
The version of the call operator that takes a mutable receiver.

Instances of FnMut can be called repeatedly and may mutate state.
 */


fn main() {
    // Calling a mutably capturing closure
    let mut s = String::new();
    let mut update_string = |str| s.push_str(str);
    update_string("hello");
    println!("{:?}", s);

    // ******************************************************************
    // Using a FnMut parameter
    fn do_twice<'a, F: FnMut(&'a str) -> ()>(mut func: F)
    {
        func("hello");
        func("world");
    }

    let mut s1 = String::new();
    let update_string1 = |pstr| s1.push_str(pstr);
    do_twice(update_string1);
    println!("{}", s1);
}