// Matching Named Variables

#[test]
fn t0() {
    let x = Some(5);
    let y = 10;

    match x {
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {y}"), // 匹配任意Option<i32>的Some变体并将Some中包含的值绑定(复制,i32实现了Copy trait)到局部变量y上

        // If x had been a None value instead of Some(5), the patterns in the first two arms wouldn’t have matched, so the value would have matched to the underscore.
        _ => println!("Default case, x = {:?}", x),
    }

    // When the match expression is done, its scope ends, and so does the scope of the inner y.
    println!("at the end: x = {:?}, y = {y}", x); // print->at the end: x = Some(5), y = 10
}

#[test]
fn t1() {
    let e_string = Some(String::from("hello rust"));
    match e_string {
        Some(s) => println!("{}", s), // 移动e_string(String没有实现Copy trait)
        None => println!("None"),
    }
    // error[E0382]: borrow of partially moved value: `e_string`
    // println!("{:?}:", e_string);

    #[derive(Debug, Copy, Clone)]
    #[allow(warnings)]
    struct Point {
        x: i32,
        y: i32,
    }
    let e_struct = Some(Point { x: 10, y: 20 });
    match e_struct {
        Some(s) => println!("{:?}", s), // 复制e_struct(Point实现了Copy trait)
        None => println!("None"),
    }
    println!("{:?}", e_struct);
}

#[test]
fn t2() {
    let e_string = Some(String::from("hello rust"));
    /*
    Bind by reference during pattern matching.

    ref annotates pattern bindings to make them borrow rather than move. It is not a part of the pattern as far as matching is concerned: it does not affect whether a value is matched, only how it is matched.
    Using the ref keyword, the value is only borrowed, not moved, making it available for use after the match statement:

    & vs ref
    * & denotes that your pattern expects a reference to an object. Hence & is a part of said pattern: &Foo matches different objects than Foo does.
    * ref indicates that you want a reference to an unpacked value. It is not matched against: Foo(ref foo) matches the same objects as Foo(foo).
     */
    match e_string {
        Some(ref s) => println!("{}", s),
        None => println!("None"),
    }
    println!("{:?}:", e_string);

    let tuple = (1, String::from("hello"));
    #[allow(warnings)]
    let (num, ref msg) = tuple;
    println!("{:?}", tuple)
}
