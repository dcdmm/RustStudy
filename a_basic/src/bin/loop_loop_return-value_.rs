// 从loop循环中返回值

fn main() {
    /*
    When associated with a loop, a break expression may be used to return a value from that loop, via one of the forms break EXPR or break 'label EXPR, where EXPR is an expression whose result is returned from the loop.
    */
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    println!("The result is {result}");
}
