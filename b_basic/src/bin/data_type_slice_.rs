// 切片(Primitive Type slice)

fn main() {
    let s = String::from("hello world");

    let _s0: &str = &s[0..2];
    let _s1 = &s[..2]; // 与上等价

    let len = s.len();
    let _ = &s[3..len];
    let _ = &s[3..]; // 与上等价

    let _ = &s[0..len];
    let _ = &s[..]; // 与上等价

    let a = [1, 2, 3, 4, 5];
    // This slice has the type &[i32]. It works the same way as string slices do, by storing a reference to the first element and a length.
    let array_slice0 = &a[1..3];
    let array_slice1: &[i32] = &[2, 3];
    let is_equal = array_slice0 == array_slice1;
    println!("{}", is_equal);
}
