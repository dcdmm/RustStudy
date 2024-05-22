// 类型推断与trubofish语法

fn create_default<T: Default>() -> T {
    T::default()
}

pub enum Result1<T, E> {
    Ok(T),
    Err(E),
}

fn main() {
    let _x_infer = create_default::<i32>(); // 推断create_default中T的类型为i32
    let _x_turbofish: i32 = create_default(); // turbofish语法;显式指定create_default中T的类型为i32

    let _y_infer: Result1<i32, &str> = Result1::Ok(34); // 推断Result1中T的类型i32,E的类型为&str;
    let _y_turbofish = Result1::Ok::<u8, &str>(34); // 显式指定Result1中T的类型i32,E的类型为&str;
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
        s0.send(1).unwrap(); // 推断mpsc::channel中T的类型为i32
    });
    handle.join().unwrap();
    let received = r0.recv().unwrap();

    let (s1, r1): (Sender<i32>, Receiver<i32>) = mpsc::channel(); // 推断mpsc::channel中T的类型为i32

    let (s2, r2) = mpsc::channel::<i32>(); // 显式指定mpsc::channel中T的类型为i32
}

#[allow(warnings)]
#[test]
fn t2() {
    let a = [1, 2, 3];
    let a_vec_infef: Vec<&i32> = a.iter().collect(); // 推断Trait std::iter::Iterator collect方法中B的类型为Vec<&i32>
    let a_vec_turbofish = a.iter().collect::<Vec<&i32>>(); // 显示指定Trait std::iter::Iterator collect方法中B的类型为Vec<&i32>
}
