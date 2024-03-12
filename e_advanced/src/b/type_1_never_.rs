// The Never Type that Never Returns

/*
The ! type, also called “never”.

! represents the type of computations which never resolve to any value at all. For example, the exit function fn exit(code: i32) -> ! exits the process without ever returning, and so returns !.

break, continue and return expressions also have type !.
 */

#[test]
fn t0() {
    let i = 2;

    // 报错:`match` arms have incompatible types
    // let _v = match i {
    //     0..=3 => i,
    //     _ => println!("其他值:{}", i)
    // };

    let _v: u32 = match i {
        0..=3 => i,
        /*
        Both match arms must produce values of type u32, but since break never produces a value at all we know it can never produce a value which isn’t a u32.
        This illustrates another behaviour of the ! type - expressions with type ! will coerce into any other type.
         */
        _ => panic!("不合规定的值:{}", i)
    };
}