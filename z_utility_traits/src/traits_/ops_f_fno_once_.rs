// Trait std::ops::FnOnce

/*
The version of the call operator that takes a by-value receiver.

Instances of FnOnce can be called, but might not be callable multiple times.
Because of this, if the only thing known about a type is that it implements FnOnce, it can only be called once.

签名:pub trait FnOnce<Args: Tuple> {
 */

#[test]
fn t0() {
    fn consume_with_relish<F>(func: F)
        where F: FnOnce() -> String
    {
        // `func` consumes its captured variables, so it cannot be run more than once.
        println!("Consumed: {}", func());

        // 报错:error[E0382]: use of moved value: `func`
        // println!("Consumed: {}", func());
    }

    let x = String::from("xXx");
    let consume_and_return_x = || x;
    consume_with_relish(consume_and_return_x);

    // 报错:borrow of moved value: `x`
    // println!("{}", x);
}