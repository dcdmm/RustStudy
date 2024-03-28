// Function std::any::type_name

// Returns the name of a type as a string slice.

use std::any::type_name;

struct Pair<T> {
    #[allow(dead_code)]
    a: T,
    #[allow(dead_code)]
    b: T,
}

fn print_type_name<T>(_val: &T) {
    println!("{}", type_name::<T>())
}


#[test]
fn t0() {
    println!("{}", type_name::<i32>()); // i32
    println!("{}", type_name::<(f64, char)>()); // (f64, char)
}

#[test]
fn t1() {
    let p1 = Pair { a: 3, b: 9 };
    let p2 = Pair { a: true, b: false };
    print_type_name(&p1); // z_std::any_type_name::Pair<i32>
    print_type_name(&p2); // z_std::any_type_name::Pair<bool>
}

#[test]
fn t2() {
    let mut v1 = Vec::new();
    v1.push(1);
    let mut v2 = Vec::new();
    v2.push(false);
    print_type_name(&v1); // alloc::vec::Vec<i32>
    print_type_name(&v2); // alloc::vec::Vec<bool>
}