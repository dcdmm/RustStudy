// 函数/()(Primitive Type unit)

/*
The () type, also called “unit”.

The () type has exactly one value (), and is used when there is no other meaningful value that could be returned. 
*/

// In function signatures, you must declare the type of each parameter.
fn sum_i_and_j(i: i32, j: i32) -> i32 {
    // You can return early from a function by using the return keyword and specifying a value, but most functions return the last expression implicitly.

    // return i + j;
    i + j // 表达式
}

// fn another_function() -> () { 
fn another_function() { // 与上等价,返回值为()时可以省略`-> ()`
    println!("Another function.");
}

fn main() {
    let result = sum_i_and_j(3, 5);
    println!("result = {result}");

    another_function();
}
