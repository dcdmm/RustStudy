// 闭包

fn main() {
    let x = 4;

    // 报错:can't capture dynamic environment in a fn item:
    // fn equal_to_x(z: i32) -> bool {
    //     z == x
    // }
    // let _is_equal_func= equal_to_x(4);

    // 闭包拥有一项函数所不具备的特性:获取作用域中变量
    let equal_to_x_c = |z| { z == x };
    println!("{}", equal_to_x_c(4));
    println!("{}", x);
}