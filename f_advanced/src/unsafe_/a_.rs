// Dereferencing a Raw Pointer

/*
Different from references and smart pointers, raw pointers:

Are allowed to ignore the borrowing rules by having both immutable and mutable pointers or multiple mutable pointers to the same location
Aren’t guaranteed to point to valid memory
Are allowed to be null
Don’t implement any automatic cleanup
*/

#[test]
fn t0() {
    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    // error[E0133]: dereference of raw pointer is unsafe and requires unsafe function or block
    // println!("r1 is: {}", *r1);

    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }
}
