// 模式匹配(匹配Option)

fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        // i绑定了Some中包含的值
        Some(i) => Some(i + 1)
    }
}
fn main() {
    let five = Some(5);
    let _six = plus_one(five);
    let _none = plus_one(None);
}