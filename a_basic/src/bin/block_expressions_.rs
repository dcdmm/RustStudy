// Block expressions

// The type of a block is the type of the final operand, or () if the final operand is omitted.

fn fn_call() {}

fn main() {
    let _: () = {
        fn_call();
    };

    let five: i32 = {
        fn_call();
        5
    };
    println!("{}", five); // print->five

    let condition = true;
    let number = if condition
    // 代码块
    {
        fn_call();
        "five"
    } else
    // 代码块
    {
        fn_call();
        "sex"
    };
    println!("{}", number); // print->five
}
