// 数组(Primitive Type array)

fn main() {
    // 特点:元素必须有相同的类型,长度固定,依次线性排列
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    // 使用索引访问数组中的元素
    let first = a[0];
    println!("{first}");

    // let _b = [3, 3, 3, 3, 3, 3]
    let _b = [3, 6]; // 与上等价,单更加简洁
}
