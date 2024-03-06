#[test]
fn t0() {
    struct Point {
        x: i32,
        y: i32,
    }
    let p = Point { x: 0, y: 7 };
    #[allow(warnings)]
    let Point { x, y } = &p;

    let s = &Some(String::from("hello rust"));
    #[allow(warnings)]
    if let Some(sx) = s {
    }

    let t = &mut (String::from("hello rust"), 1);
    #[allow(warnings)]
    let (tx, ty) = t;

    let a = &mut [String::from("hello"), String::from("rust")];
    #[allow(warnings)]
    let [ax, ay] = a;
}