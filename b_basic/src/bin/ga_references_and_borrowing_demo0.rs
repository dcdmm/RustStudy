#![allow(warnings)]

// 引用与借用常见错误解析0 

#[test]
fn t0() {
    // example0:
    let a = 34;
    let b = &a;
    let c = b; // 复制不可变引用(不可变引用可以有任意个)
    println!("{a}, {b}, {c}");

    // example1:
    // let mut x = 41;
    // let y = &mut x;    
    // let z = y; // 移动可变引用(可变引用只能有一个)
    // // error[E0382]: borrow of moved value: `y`
    // println!("{}", y); // `y`: value borrowed here after move
}

#[test]
fn t1() {
    // example0:
    // let mut x = 41;
    // let xr = &mut x;  // `&mut x``:`x` is borrowed here
    // // error[E0506]: cannot assign to `x` because it is borrowed
    // x = 100; // `x = 100`: `x` is assigned to here but it was already borrowed
    // println!("{}", xr); // `{xr}`: borrow later used here

    // example1:
    // let mut y = 34;
    // let yr = &y; // `&y`: `y` is borrowed here
    // // error[E0506]: cannot assign to `y` because it is borrowed
    // y = 100; // `y=100`: `y` is assigned to here but it was already borrowed
    // println!("{}", yr); // `yr`: borrow later used here
}

#[test]
fn t2() {
    // example0:
    // let x = String::from("hello world");
    // let xr = &x;
    // // error[E0507]: cannot move out of `*xr` which is behind a shared reference
    // let xr_def = *xr; // `*xr`: move occurs because `*xr` has type `String`, which does not implement the `Copy` trait

    // example1:
    // let mut y = String::from("hello world");
    // let yr = &mut y;
    // // error[E0507]: cannot move out of `*yr` which is behind a mutable reference
    // let yr_def = *yr; // `*yr`: move occurs because `*yr` has type `String`, which does not implement the `Copy` trait

    // example2:
    // let mut z = String::from("hello world");
    // let zr = &z;
    // // error[E0505]: cannot move out of `z` because it is borrowed
    // let z_move = z; // `z`: move out of `z` occurs here
    // println!("{}", zr); // `zr`: borrow later used here
}

#[test]
fn t3() {
    struct Point{
        x: String,
        y: i32,
    }

    // example0:
    // let mut p = Point{x:String::from("hello"), y:3};
    // let pr = &mut p;
    // // error[E0507]: cannot move out of `pr.x` which is behind a mutable reference
    // let x0 = pr.x; // `pr.x`: move occurs because `pr.x` has type `String`, which does not implement the `Copy` trait

    // example1:
    // let mut q = Point{x:String::from("hello"), y:3};
    // let qr = &q;
    // // error[E0507]: cannot move out of `qr.x` which is behind a shared reference
    // let x1 = qr.x;  // `qr.x`: move occurs because `qr.x` has type `String`, which does not implement the `Copy` trait

    // example2:
    // let mut m = Point{x:String::from("hello"), y:3};
    // let mr = &m; // `&m`: borrow of `m` occurs here
    // // error[E0505]: cannot move out of `m.x` because it is borrowed
    // let x2 = m.x; // `m.x`: move out of `m.x` occurs here
    // println!("{}", mr.x); // `mr.x`: borrow later used here
}

fn main() {}
