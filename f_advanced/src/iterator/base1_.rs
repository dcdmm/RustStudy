// 其他支持迭代的类型

#[test]
fn hash_set_test() {
    // std::ops::Range
    for i in 1..3 { 
        print!("{}\t", i)
    }
    println!();

    // std::ops::RangeInclusive
    for i in 1..=3 {
        print!("{}\t", i)
    }

    // Option<T>
    let s = Some(10);
    let _sii = s.into_iter();

    // Result<T, E>
    let r: Result<u32, &str> = Ok(2);
    let _rii = r.into_iter();
}
