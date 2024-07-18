#![allow(warnings)]

// 引用与借用常见错误解析1

#[test]
fn t0() {
    #[derive(Debug)]
    struct Point {
        x: i32,
        y: i32,
    }

    // example0:
    let mut p = Point { x: 1, y: 2 };
    let px_ref = &mut p.x;
    let py_ref = &mut p.y;
    *px_ref += 1;
    *py_ref += 1;
    println!("{:?}", p); // print->Point { x: 2, y: 3 }

    // example1:
    // let mut q = Point { x: 1, y: 2 };
    // let qx_ref = &mut q.x; // `&mut q.x`: first mutable borrow occurs here
    // let q_ref = &mut q; // `&mut q`: second mutable borrow occurs here
    // // error[E0499]: cannot borrow `q` as mutable more than once at a time
    // *qx_ref += 1; // `*qx_ref += 1`: first borrow later used here
}

fn main() {}
