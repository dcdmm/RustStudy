// std::collections::VecDeque(参考C++ deque;参考 Java ArrayDeque;参考Python collections.deque)

// A double-ended queue implemented with a growable ring buffer.

use std::collections::VecDeque;

#[test]
fn t0() {
    let deq = VecDeque::from([-1, 0, 1]);
    println!("{:?}", deq);

    let vd1: VecDeque<u32> = VecDeque::new(); // 声明vd1 Type annotations为VecDeque<i32>(编译器不能进行推断)
    println!("{:?}", vd1);

    let mut vd11 = VecDeque::new();
    vd11.push_back(1); // 推断vd11的类型为VecDeque<i32>
}

#[test]
fn t1() {
    let v = VecDeque::from([-1, 0, 1]);
    println!("{}", v[1]); // 索引;print->0

    // panicked: Out of bounds access
    println!("{}", v[5]);
}
