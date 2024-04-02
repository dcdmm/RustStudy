// Function std::any::type_name

// Returns the name of a type as a string slice.

use std::any::type_name;

#[allow(warnings)]
struct Pair<T> {
    a: T,
    b: T,
}

#[allow(warnings)]
fn print_type_name<T>(_val: &T) {
    println!("{}", type_name::<T>())
}

#[test]
fn t0() {
    println!("{}", type_name::<i32>()); // print->i32
    println!("{}", type_name::<(f64, char)>()); // print->(f64, char)

    let mut v1 = Vec::new();
    v1.push(1);
    let mut v2 = Vec::new();
    v2.push(false);
    print_type_name(&v1); // print->alloc::vec::Vec<i32>
    print_type_name(&v2); // print->alloc::vec::Vec<bool>
}

#[test]
fn t1() {
    let p1 = Pair { a: 3, b: 9 };
    let p2 = Pair { a: true, b: false };
    print_type_name(&p1); // print->z_std_other::functions::any_type_name::Pair<i32>
    print_type_name(&p2); // print->z_std_other::functions::any_type_name::Pair<bool>
}
