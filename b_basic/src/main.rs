fn main() {

    let mut a = 34;
    let b = &mut a;
    let c = b;
    println!("{b}");
    // println!("{c}");
    // let number = 10;
    // let some_number = Some(number);

    // match &&&&&&some_number {
    //     &&&&&&Some(s) => {
    //         println!("{}", std::any::type_name_of_val(&s));
    //     },
    //     None => println!("没有匹配到任何数字"),
    // }

    // struct p{
    //     x: String,
    //     y: i32,
    // }

    // let mut  p0 = p{x:String::from("hello"), y:3};
    // let p0r =  &mut p0;
    // let x1 = p0.x;
    // println!("{}", p0.x);

    // // let qq = String::from("value");
    // // let qqr = &qq;
    // // let y2 = *qqr;
    // // println!("{}", qqr);

    // println!("{}", 34);

    // let a1:i32;
    // let b = &a1;

    // let s1 = String::from("3");
    // let s2 = &s1;
    // let q = s1;
    // println!("{}", s2)

    // println!("{}", p0.x);
    // println!("{}", (*p0r).x);
    // let int_reference = &3;

    // let a = match *int_reference {
    //     0 => "zero",
    //     _ => "some",
    // };
    // let b = match int_reference {
    //     &0 => "zero",
    //     _ => "some",
    // };

    // let a = 234;
    // let b = &a;
    // let c  = &b;
    // let d = &c;
    // println!("{}", std::any::type_name_of_val(&a));
    // println!("{}", std::any::type_name_of_val(&b));
    // println!("{}", std::any::type_name_of_val(&d));
}
