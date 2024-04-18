// 使用关联类型进行Trait Bound约束

#[test]
fn t0() {
    fn add<T>(a: T, b: T) -> T
    where
        // 约束特质Add的关联类型为T类型
        T: std::ops::Add<Output = T>,
    {
        a + b
    }

    let result = add(1, 2);
    println!("{}", result);
}

#[test]
fn t1() {
    use std::ops::{Add, Sub};

    fn arithmetic_operations<T>(x: T, y: T) -> (T, T)
    where
        // 约束特质Add、Sub的关联类型为T类型
        T: Add<Output = T> + Sub<Output = T> + Copy,
    {
        let sum = x + y;
        let difference = x - y;
        (sum, difference)
    }

    let a = 10;
    let b = 5;
    let (sum, difference) = arithmetic_operations(a, b);
    println!("Sum: {}, Difference: {}", sum, difference);
}
