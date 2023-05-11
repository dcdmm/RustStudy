// 数组(Primitive Type array)

// An array is a fixed-size sequence of N elements of type T. The array type is written as [T; N].

/*
There are two syntactic forms for creating an array:
* A list with each element, i.e., [x, y, z].
* A repeat expression [x; N], which produces an array with N copies of x. The type of x must be Copy.
 */

#[test]
fn t0() {
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    // ★★★★★Arrays coerce to slices ([T]), so a slice method may be called on an array. Indeed, this provides most of the API for working with arrays.
    println!("{}", a.len());  // Returns the number of elements in the slice.

    let first = a[0]; // 通过索引访问数组中的元素
    println!("{first}");

    // let _b = [3, 3, 3, 3, 3, 3]
    let _b = [3, 6]; // 与上等价,但更加简洁

    for i in a { // 数组实现了`Trait std::iter::IntoIterator`
        println!("{i}")
    }

    for j in 0..a.len() {
        println!("{}", a[j])
    }
}
