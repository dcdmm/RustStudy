#[test]
fn t0() {
    use x_cargo_feature::Point;

    let p = Point { x: 1, y: 2 };
    // 已开启display,impl Display参与编译,可以直接使用{p}
    println!("{p}"); // print-><<<1, 2>>>
}
