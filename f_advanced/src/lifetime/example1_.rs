// 不同的生命周期参数

#[allow(warnings)]
#[test]
fn t0() {
    // 结构体S的两个字段是具有相同生命周期'a的引用
    struct S<'a> {
        x: &'a i32,
        y: &'a i32,
    }

    // let x = 10;
    // let r;
    // {
    //     let y = 20; // `y`: binding `y` declared here
    //     {
    //         // 要求'a不能长于y的生命周期
    //         // error[E0597]: `y` does not live long enough
    //         let s = S { x: &x, y: &y }; // `&y`:borrowed value does not live long enough
    //         // 'a覆盖r的生命周期
    //         r = s.x;
    //     }
    // } // `}`: dropped here while still borrowed
    // // 生命周期不能满足比y短但比r长
    // println!("{}", r); // `r`: borrow later used here
}

#[allow(warnings)]
#[test]
fn t1() {
    struct S<'a, 'b> {
        x: &'a i32,
        y: &'b i32,
    }
    let x = 10;
    let r;
    {
        let y = 20;
        {
            let s = S { x: &x, y: &y }; // 根据结构体定义,s.x和s.y具有独立的生命周期
            r = s.x;
        }
    }
    println!("{}", r);
}
