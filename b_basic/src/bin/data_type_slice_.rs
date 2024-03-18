// 切片(Primitive Type slice)

fn main() {
    let s = String::from("hello world");

    let _s0: &str = &s[0..2]; // 字符串切片的引用
    let _s0_: &str = &s[std::ops::Range { start: 0, end: 2 }]; // 与上等价

    let a = [1, 2, 3, 4, 5];
    // This slice has the type &[i32]. It works the same way as string slices do, by storing a reference to the first element and a length.
    let array_slice0 = &a[1..3]; // 切片的引用(类型为&[i32])
    let array_slice1: &[i32; 2] = &[2, 3]; // 数组的引用(类型为&[i32; 2])
    let is_equal = array_slice0 == array_slice1;
    println!("{}", is_equal); // print->true

    let _index = array_slice0[0]; // 索引操作(usize类型)
    let _slice = &array_slice0[..2]; // 切片操作(usize类型)
}
