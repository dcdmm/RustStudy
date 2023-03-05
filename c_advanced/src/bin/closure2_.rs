// 闭包(FnOnce/FnMut/Fn)

// Closures can capture values from their environment in three ways, which directly map to the three ways a function can take a parameter: borrowing immutably, borrowing mutably, and taking ownership. The closure will decide which of these to use based on what the body of the function does with the captured values.

// The way a closure captures and handles values from the environment affects which traits the closure implements, and traits are how functions and structs can specify what kinds of closures they can use. Closures will automatically implement one, two, or all three of these Fn traits, in an additive fashion, depending on how the closure’s body handles the values:

// FnOnce applies to closures that can be called once. All closures implement at least this trait, because all closures can be called. A closure that moves captured values out of its body will only implement FnOnce and none of the other Fn traits, because it can only be called once.
// FnMut applies to closures that don’t move captured values out of their body, but that might mutate the captured values. These closures can be called more than once.
// Fn applies to closures that don’t move captured values out of their body and that don’t mutate captured values, as well as closures that capture nothing from their environment. These closures can be called more than once without mutating their environment, which is important in cases such as calling a closure multiple times concurrently.

// 创建闭包时,通过闭包对作用域变量的使用,Rust推断出具体使用哪个trait:
// 1. 所有闭包都实现了FnOnce
// 2. 没有移出作用域变量所有权的闭包实现了FnMut
// 3. 不需要对作用域变量进行修改(即只需要获取作用域变量的不可变借用)的实现了Fn
// ======>总结:Fn 的前提是实现FnMut,FnMut的前提是实现FnOnce
// ======>总结:一个闭包实现了哪种Fn特征取决于该闭包如何使用被捕获的变量,而不是取决于闭包如何捕获它们

fn fn_once<F>(func: F)
where
    // 获取作用域变量所有权
    F: FnOnce(usize) -> bool,
{
    println!("{}", func(3));
    // println!("{}", func(4));
    // 程序运行结果:
//     error[E0382]: use of moved value: `func`
//   --> c_advanced\src\bin\closure2_.rs:15:20
//    |
// 18 | fn fn_once<F>(func: F)
//    |               ---- move occurs because `func` has type `F`, which does not implement the `Copy` trait
// ...
// 23 |     println!("{}", func(3));
//    |                    ------- `func` moved due to this call
// 24 |     println!("{}", func(4));
//    |                    ^^^^ value used here after move
//    |
// note: this value implements `FnOnce`, which causes it to be moved when called
//   --> c_advanced\src\bin\closure2_.rs:14:20
//    |
// 23 |     println!("{}", func(3));
//    |                    ^^^^
// help: consider further restricting this bound
//    |
// 21 |     F: FnOnce(usize) -> bool + Copy,
//    |                              ++++++
}

// 获取作用域变量的可变借用
fn exec<'a, F: FnMut(&'a str)>(mut f: F) {
    f("hello")
}

// 获取作用域变量的不可变借用
fn exec1<'a, F: Fn(String) -> ()>(f: F)  {
    f("world".to_string())
}

fn main() {
    let x = vec![1, 2, 3];
    fn_once(|z|{z == x.len()});
    
    // ********************************************
    let x1 = vec![1, 2, 3];
    // 强制闭包获取作用域变量的所有权
    let fn_once1 = move || x1;
    println!("{:?}", fn_once1());
    // println!("{:?}", x1);
    // 程序运行结果:
//     error[E0382]: borrow of moved value: `x1`
//     --> c_advanced\src\bin\closure2_.rs:54:22
//      |
//   63 |     let x1 = vec![1, 2, 3];
//      |         -- move occurs because `x1` has type `Vec<i32>`, which does not implement the `Copy` trait
//   64 |     // 强制闭包获取作用域变量的所有权
//   65 |     let x1_move = move || x1;
//      |                   ------- -- variable moved due to use in closure
//      |                   |
//      |                   value moved into closure here
//   66 |     println!("{:?}", x1_move());
//   67 |     println!("{:?}", x1);
//      |                      ^^ value borrowed here after move

    println!("************************************************");
    let mut s = String::new();

    // 想要在闭包内部获取作用域变量的可变借用,需要把该闭包声明为可变类型
    let mut update_string =  |str| s.push_str(str);
    update_string("hello");
    println!("{:?}",s);

    // ********************************************
    let mut s1 = String::new();

    let update_string1 =  |str| s1.push_str(str);
    exec(update_string1);
    println!("{:?}", s1);

    println!("************************************************");
    let m = "hello, ".to_string();

    let print_string =  |str| println!("{},{}", m, str);
    exec1(print_string);
    println!("{:?}", m);
}