// Box常见错误解析 

#[derive(Debug)]
struct People {
    age: i32,
}

#[test]
fn t1() {
    let p0 = People { age: 10 };
    let mut p0_box = Box::new(p0); // `p0`: value moved here 
    // error[E0382]: borrow of moved value: `p0`
    // note: move occurs because `*a0box` has type `a`, which does not implement the `Copy` trait
    // println!("{:?}", p0);
}

#[test]
fn t2() {
    let p0 = People { age: 10 };
    let p0_box = Box::new(p0); 

    // let p0_box_deref = *p0_box; // `*p0_box`: value moved here
    // error[E0382]: borrow of moved value: `p0_box`
    // note: move occurs because `*p0_box` has type `Box1_::People`, which does not implement the `Copy` trait
    // println!("{:?}", p0_box);

    let p0_box_deref = &*p0_box; // 对p0_box所拥有的那块堆内存的借用
    // println!("{:?}", p0_box)
}

#[test]
fn t3() {
    let p0 = People { age: 10 };
    let p0_box = Box::new(p0); 
    // error[E0594]: cannot assign to `p0_box.age`, as `p0_box` is not declared as mutable
    // (*p0_box).age = 100;

    let p1 = People { age: 10 };
    let mut p1_box = Box::new(p1);
    p1_box.age = 100; // 自动解引用
    println!("{:?}", p1_box); // print->People { age: 100 }
}