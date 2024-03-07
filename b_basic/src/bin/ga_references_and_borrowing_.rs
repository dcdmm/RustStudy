// 引用与借用错误解析 

#[allow(warnings)]
#[test]
fn t0() {
    let a = 34;
    let b = &a;
    let c = b; // 复制不可变引用(不可变引用可以有任意个)
    println!("{a}, {b}, {c}");

    let mut x = 41;
    let y = &mut x;    
    let z = y; // 移动可变引用(可变引用只能有一个)
    // error[E0382]: borrow of moved value: `y`
    // println!("{y}"); // {y}:value borrowed here after move
}

#[allow(warnings)]
#[test]
fn t1() {
    let mut x = 41;
    let y = &mut x;  // &mut x:`x` is borrowed here
    // error[E0506]: cannot assign to `x` because it is borrowed
    // x = 100; // `x` is assigned to here but it was already borrowed
    println!("{y}"); // {y}:borrow later used here

    let mut a = 34;
    let b = &a; // &a:`a` is borrowed here
    // error[E0506]: cannot assign to `a` because it is borrowed
    // a = 100; // `a` is assigned to here but it was already borrowed
    println!("{b}"); // {b}:borrow later used here
}

#[allow(warnings)]
#[test]
fn t2() {
    struct Point{
        x: String,
        y: i32,
    }

    let mut p = Point{x:String::from("hello"), y:3};
    let pr = &mut p;
    // error[E0507]: cannot move out of `pr.x` which is behind a mutable reference
    // let x1 = pr.x; // pr.x:move occurs because `pr.x` has type `String`, which does not implement the `Copy` trait

    let mut q = Point{x:String::from("hello"), y:3};
    let qr = &q;
    // error[E0507]: cannot move out of `qr.x` which is behind a shared reference
    // let x1 = qr.x;
}

fn main() {}
