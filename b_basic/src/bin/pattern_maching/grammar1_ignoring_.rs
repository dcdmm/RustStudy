// 模式匹配(Ignoring Values in a Pattern)

#[allow(dead_code)]
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

// Ignoring an Entire Value with _
#[allow(warnings)]
fn foo(_: i32, y: i32) {
    println!("This code only uses the y parameter: {}", y);
}

#[test]
fn t0() {
    foo(3, 4);

    // ***************************************************
    let numbers = (2, 4, 8, 16, 32);

    // Ignoring Parts of a Value with a Nested _
    match numbers {
        (first, _, third, _, fifth) => {
            //  Ignoring multiple parts of a tuple
            println!("Some numbers: {first}, {third}, {fifth}")
        }
    }

    // ***************************************************
    /*
    If you create a variable but don’t use it anywhere, Rust will usually issue a warning because an unused variable could be a bug.
    However, sometimes it’s useful to be able to create a variable you won’t use yet, such as when you’re prototyping or just starting a project. In this situation, you can tell Rust not to warn you about the unused variable by starting the name of the variable with an underscore.
     */
    let _x = 5;
    // 警告:unused variable: `y`
    // let y = 10;

    /*
    Note that there is a subtle difference between using only _ and using a name that starts with an underscore.
    The syntax _x still binds the value to the variable, whereas _ doesn’t bind at all.
     */
    let s0 = Some(String::from("Hello!"));
    if let Some(_s0) = s0 { // s value will still be moved into _s, which prevents us from using s again.
        println!("found a string");
    }
    // 报错: borrow of partially moved value: `s0`
    // println!("{:?}", s0);

    let s1 = Some(String::from("Hello!"));
    if let Some(_) = s1 { // using the underscore by itself doesn’t ever bind to the value
        println!("found a string");
    }

    // ***************************************************

    let origin = Point { x: 0, y: 0, z: 0 };

    /*
    Ignoring Remaining Parts of a Value with ..

    With values that have many parts, we can use the .. syntax to use specific parts and ignore the rest, avoiding the need to list underscores for each ignored value.
    The .. pattern ignores any parts of a value that we haven’t explicitly matched in the rest of the pattern.
     */
    match origin {
        Point { x, .. } => println!("x is {}", x),
    }

    let numbers = (2, 4, 8, 16, 32);

    match numbers {
        (first, .., last) => {
            println!("Some numbers: {first}, {last}");
        }
    }
}
