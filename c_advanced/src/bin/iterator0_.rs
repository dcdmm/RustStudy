// 迭代器

#[cfg(test)]
mod tests {
    #[test]
    fn iterator_demonstration() {
        let v1 = vec![1, 2, 3];

        let mut v1_iter = v1.iter();
        
        // 返回Option类型====>有值时返回Some(i32),无值时返回None
        // 依次取出迭代器中的元素
        // Note that we needed to make v1_iter mutable: calling the next method on an iterator changes internal state that the iterator uses to keep track of where it is in the sequence. In other words, this code consumes, or uses up, the iterator. Each call to next eats up an item from the iterator. We didn’t need to make v1_iter mutable when we used a for loop because the loop took ownership of v1_iter and made it mutable behind the scenes.
        assert_eq!(v1_iter.next(), Some(&1));
        assert_eq!(v1_iter.next(), Some(&2));
        assert_eq!(v1_iter.next(), Some(&3));
        assert_eq!(v1_iter.next(), None);
    }
}

fn main() {
    let values = vec![1, 2, 3];

    // into_iter:创建的迭代器获取values所有权
    for v in values.into_iter() {
        println!("{}", v)
    }
    // println!("{:?}", values);
    // 程序运行结果:
//     error[E0382]: borrow of moved value: `values`
//    --> c_advanced\src\bin\iterator0_.rs:24:22
//     |
// 22  |     let values = vec![1, 2, 3];
//     |         ------ move occurs because `values` has type `Vec<i32>`, which does not implement the `Copy` trait
// 25  |
// 25  |     for v in values.into_iter() {
//     |                     ----------- `values` moved due to this method call
// ...
// 28  |     println!("{:?}", values);
//     |                      ^^^^^^ value borrowed here after move
//     |
// note: this function takes ownership of the receiver `self`, which moves `values`
//    --> C:\Users\dcdmm\.rustup\toolchains\stable-x86_64-pc-windows-msvc\lib/rustlib/src/rust\library\core\src\iter\traits\collect.rs:261:18
//     |
// 261 |     fn into_iter(self) -> Self::IntoIter;
//     |                  ^^^^

    let values1 = vec![1, 2, 3];
    // iter:创建的迭代器获取values1的不可变借用
    let _values_iter = values1.iter();
    println!("{:?}", values1);

    let mut values2 = vec![1, 2, 3];
    // iter:创建的迭代器获取values2的可变借用
    let mut values_iter_mut = values2.iter_mut();
    if let Some(v) = values_iter_mut.next() {
        *v = 0;
    }
    println!("{:?}", values2);
}

