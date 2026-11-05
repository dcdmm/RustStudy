// 类型推断与 turbofish 语法

fn create_default<T: Default>() -> T {
    T::default()
}

pub enum Result1<T, E> {
    Ok(T),
    Err(E),
}

fn main() {
    // 通过turbofish显式指定函数create_default泛型参数T为i32
    let _x_turbofish = create_default::<i32>();
    // 通过显式标注变量类型,推断函数create_default泛型参数T为i32
    let _x_infer: i32 = create_default();

    // 通过显式标注变量类型,推断Result1的泛型参数T为i32、E为&str
    let _y_infer: Result1<i32, &str> = Result1::Ok(34);
    // 通过turbofish显式指定Result1的泛型参数T为 u8、E为&str
    let _y_turbofish = Result1::Ok::<u8, &str>(34);
}

#[allow(warnings)]
#[test]
fn t0() {
    use std::sync::mpsc;
    use std::sync::mpsc::Receiver;
    use std::sync::mpsc::Sender;
    use std::thread;

    let (s0, r0) = mpsc::channel();
    let handle = thread::spawn(move || {
        // 根据发送值的类型推断 mpsc::channel 的泛型参数 T；此处整数默认类型为 i32
        s0.send(1).unwrap();
    });
    handle.join().unwrap();
    let received = r0.recv().unwrap();

    //通过显式标注变量类型,推断mpsc::channel 的泛型参数T为i32
    let (s1, r1): (Sender<i32>, Receiver<i32>) = mpsc::channel();

    // 通过turbofish显式指定mpsc::channel的泛型参数T为i32
    let (s2, r2) = mpsc::channel::<i32>();
}

#[allow(warnings)]
#[test]
fn t2() {
    let a = [1, 2, 3];
    // 通过显式标注变量类型,推断Iterator::collect的泛型参数B为Vec<&i32>
    let a_vec_infer: Vec<&i32> = a.iter().collect();
    // 通过turbofish显式指定Iterator::collect 的泛型参数B为Vec<&i32>
    let a_vec_turbofish = a.iter().collect::<Vec<&i32>>();
}
