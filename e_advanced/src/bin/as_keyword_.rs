// Keyword as

fn main() {
    let a = 3.1 as i8;
    let b = 100_i8 as f32 + 10.5;
    let c = 'a' as u8;
    println!("{},{},{}", a, b, c);

    let num = 5;
    // using as to cast an immutable and a mutable reference into their corresponding raw pointer types
    let p1 = &num as *const i32;
    println!("{:?}", p1);

    let mut values: [i32; 2] = [1, 2];
    let p1: *mut i32 = values.as_mut_ptr();
    // Pointer to address cast
    let first_address = p1 as usize;
    let second_address = first_address + 4; // 4 == size_of::<i32>()
    // Address to pointer cast
    let p2 = second_address as *mut i32;
    unsafe {
        *p2 += 1;
    }
    assert_eq!(values[1], 3);
}