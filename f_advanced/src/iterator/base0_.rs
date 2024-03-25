// 迭代器

#[test]
fn array_test() {
    let arr = [1, 2, 3];
    let fa = &arr[..];
    let a_iter = arr.iter();
    let v = vec![1, 2, 3, 4, 5];

    // 使用iter()获取一个不可变借用的迭代器
    for value in v.iter() {
        println!("{}", value);
    }
}

#[test]
fn vec_test() {
    let v1 = vec![1, 2, 3];
    let f = &v1[..];
    v1.is_empty();
    let s = v1.iter();
    // let v1_iter = v1.iter(); // Creating an iterator

    // // Using an iterator in a for loop
    // for val in v1_iter {
    //     println!("{}", val);
    // }
}

#[test]
fn xdf() {
    use std::collections::VecDeque;

    let mut buf = VecDeque::new();
    buf.push_back(5);
    buf.push_back(3);
    buf.push_back(4);
    // let b: &[_] = &[&5, &3, &4];
    // let c: Vec<&i32> = buf.iter().collect();
    // assert_eq!(&c[..], b);
}
