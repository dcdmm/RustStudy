// 类型转换(Keyword as)

// as is most commonly used to turn primitive types into other primitive types, but it has other uses that include turning pointers into addresses, addresses into pointers, and pointers into other pointers.

#[test]
fn t0() {
    let a = 3.1 as i8;
    let b = 100_i8 as f32 + 10.5;
    let c = 'a' as u8;
    let d = 108 as char;
    println!("{}, {}, {}, {}", a, b, c, d); // print->3, 110.5, 97, l

    let num = 5;
    let p1 = &num as *const i32; // creating raw pointers from references
    println!("{:?}", p1);

    let mut values: [i32; 2] = [1, 2];
    let p1: *mut i32 = values.as_mut_ptr();
    let first_address = p1 as usize; // Pointer to address cast
    let second_address = first_address + 4; // 4 == size_of::<i32>()
    let p2 = second_address as *mut i32; // Address to pointer cast
    unsafe {
        *p2 += 1;
    }
    println!("{}", values[1]);
}
