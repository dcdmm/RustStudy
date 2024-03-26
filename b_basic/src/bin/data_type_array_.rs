// 数组(Primitive Type array)

// An array is a fixed-size sequence of N elements of type T. The array type is written as [T; N].

/*
There are two syntactic forms for creating an array:
* A list with each element, i.e., [x, y, z].
* repeat expression [expr; N] where N is how many times to repeat expr in the array. expr must either be:
*   * A value of a type implementing the Copy trait
*   * A const value
 */

fn main() {
    let a: [i32; 5] = [1, 2, 3, 4, 5];

    // Arrays coerce to slices ([T]), so a slice method may be called on an array. Indeed, this provides most of the API for working with arrays.
    println!("{}", a.len());
    println!("{}", a.is_empty());

    let first = a[2]; // 索引操作(usize类型)
    println!("{first}");

    let fs = &a[0..2]; // 切片操作(usize类型)
    println!("{:?}", fs);

    // let _b = [3, 3, 3, 3, 3, 3]
    let _b = [3, 6]; // 与上等价,但更加简洁

    // 创建一个3x3的二维数组
    let two_d_array: [[i32; 3]; 3] = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];
    for i in two_d_array {
        // 数组实现了`Trait std::iter::IntoIterator`
        for j in i {
            println!("{j}");
        }
    }
    for n in 0..two_d_array.len() {
        for m in 0..two_d_array[n].len() {
            println!("{}", two_d_array[n][m]);
        }
    }
}

#[test]
fn into_iter_test() {
    let a0 = [1, 2, 3, 4];
    /*
    impl<T, const N: usize> IntoIterator for [T; N]
        fn into_iter(self) -> <[T; N] as IntoIterator>::IntoIter
    
    Creates a consuming iterator, that is, one that moves each value out of the array (from start to end). The array cannot be used after calling this unless T implements Copy, so the whole array is copied.
     */
    let a0ii = a0.into_iter();
    for i in a0ii { // 迭代类型为i32
        println!("{}", i)
    }
    println!("{:?}", a0);

    let z0 = &mut [1, 2, 3, 4];
    let _z0ii: std::slice::IterMut<'_, i32> = z0.into_iter(); // 迭代类型为&mut i32;等价于:z0.iter_mut()

    let z1 = &[1, 2, 3, 4];
    let _z0ii: std::slice::Iter<'_, i32> = z1.into_iter(); // 迭代类型为&i32;等价于:z1.iter()
}