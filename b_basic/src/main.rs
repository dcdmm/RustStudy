fn main() {
    struct Point{
        x: String,
        y: i32,
    }

    let mut p = Point{x:String::from("hello"), y:3};
    let pr = &p;
    let prr = &pr;
    println!("{}", prr.x);

    // let mut p1 = Point{x:String::from("hello"), y:3};

    // let m = (p, 1);
    // let ms = &m;
    // println!("{}", (*ms).0.x);
}
