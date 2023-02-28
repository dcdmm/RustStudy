// 模式匹配(结构元组与数组)

fn main() {
    let (tup1, tup2) = ('a', 2);
    println!("tup1:{}", tup1);
    println!("tup2:{}", tup2);

    let arr: [u16; 2] = [114, 514];
    let [arr1, arr2] = arr;
    println!("arr1:{}", arr1);
    println!("arr2:{}", arr2);
}
