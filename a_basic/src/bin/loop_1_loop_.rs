// loop循环

#[test]
fn main() {
    /*
    A loop expression repeats execution of its body continuously: loop { println!("I live."); }.

    A loop expression without an associated break expression is diverging and has type !.
    A loop expression containing associated break expression(s) may terminate, and must have type compatible with the value of the break expression(s).
    */
    loop {
        println!("I live.")
    }
}
