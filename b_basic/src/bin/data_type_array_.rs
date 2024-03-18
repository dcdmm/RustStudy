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
