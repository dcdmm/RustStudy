// 解构Option

#[allow(warnings)]
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        // i绑定了任意Option<i32>的Some变体中包含的值
        Some(i) => Some(i + 1)
    }
}

#[test]
fn t0() {
    let five = Some(5);
    let _six = plus_one(five);
    let _none = plus_one(None);
}