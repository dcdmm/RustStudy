// 模式匹配(解构元组与数组)

#[test]
fn t0() {
    let (tup1_ele, tup2_ele) = ('a', 2);
    println!("tup1:{}", tup1_ele);
    println!("tup2:{}", tup2_ele);

    let arr: [u16; 2] = [114, 514];
    let [arr1_ele, arr2_ele] = arr;
    println!("arr1:{}", arr1_ele);
    println!("arr2:{}", arr2_ele);
}
