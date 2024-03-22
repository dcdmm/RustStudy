// 迭代器

#[test]
fn t0() {
    let v1 = vec![1, 2, 3];
    let v1_iter = v1.iter(); // Creating an iterator

    // Using an iterator in a for loop
    for val in v1_iter {
        println!("{}", val);
    }
}