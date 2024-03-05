// 模式匹配语法(Ignoring Values in a Pattern)

#[allow(warnings)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

#[test]
fn t0() {
    // Ignoring an Entire Value with _
    fn foo(_: i32, y: i32) {
        println!("This code only uses the y parameter: {}", y);
    }
    foo(3, 4);

    let numbers = (2, 4, 8, 16, 32);
    // Ignoring Parts of a Value with a Nested _
    match numbers {
        (first, _, third, _, fifth) => {
            //  Ignoring multiple parts of a tuple
            println!("Some numbers: {first}, {third}, {fifth}")
        }
    }
}

#[test]
fn t1() {
    /*
    Ignoring an Unused Variable by Starting Its Name with _

    If you create a variable but don’t use it anywhere, Rust will usually issue a warning because an unused variable could be a bug. However, sometimes it’s useful to be able to create a variable you won’t use yet, such as when you’re prototyping or just starting a project. In this situation, you can tell Rust not to warn you about the unused variable by starting the name of the variable with an underscore.
     */
    let _x = 5;

    fn xxx(_a: i32, b: i32) {
        println!("This code only uses the b parameter: {}", b);
    }
    xxx(1, 2);

    // warning: unused variable: `y`
    // let y = 10;

    // warning: unused variable: `a`
    // fn yyy(a: i32, b: i32) {
    //     println!("This code only uses the b parameter: {}", b);
    // }
    // yyy(1, 2);
}

#[test]
fn t2() {
    /*
    Note that there is a subtle difference between using only _ and using a name that starts with an underscore. The syntax _x still binds the value to the variable, whereas _ doesn’t bind at all.
     */
    let s0 = Some(String::from("Hello!"));
    if let Some(_s0) = s0 {
        // s value will still be moved into _s, which prevents us from using s again.
        println!("found a string");
    }
    // error[E0382]: borrow of partially moved value: `s0`
    // println!("{:?}", s0);

    let s1 = Some(String::from("Hello!"));
    if let Some(_) = s1 {
        // using the underscore by itself doesn’t ever bind to the value
        println!("found a string");
    }
    println!("{:?}", s1)
}

#[test]
fn f3() {
    let origin = Point { x: 0, y: 0, z: 0 };
    /*
    Ignoring Remaining Parts of a Value with ..

    With values that have many parts, we can use the .. syntax to use specific parts and ignore the rest, avoiding the need to list underscores for each ignored value. The .. pattern ignores any parts of a value that we haven’t explicitly matched in the rest of the pattern.
     */
    match origin {
        Point { x, .. } => println!("x is {}", x), // 忽略x之后所有的字段
    }

    let numbers = (2, 4, 8, 16, 32);
    match numbers {
        (first, second,  .., last) => { // 仅匹配第一个 、第二个和最后一个值
            println!("{first}, {second}, {last}");
        }
    }
    match numbers {
        (.., last) => println!("{last}") // 仅匹配最后一个值
    }
}
