// 闭包

fn main() {
    let x = 4;

    // fn equal_to_x(z: i32) -> bool {
    //     z == x
    // }
    // let _is_equal_func= equal_to_x(4);
    // 程序运行结果:
//     error[E0434]: can't capture dynamic environment in a fn item
//  --> c_advanced\src\bin\closure1_.rs:7:14
//   |
// 7 |         z == x
//   |              ^
//   |
//   = help: use the `|| { ... }` closure form instead
    
    // 闭包拥有一项函数所不具备的特性:获取作用域中的变量
    let equal_to_x_c = |z| {z == x};
    let _is_equal_closure = equal_to_x_c(4);
}