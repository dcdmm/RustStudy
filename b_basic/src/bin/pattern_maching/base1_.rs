// while let Conditional Loops;for Loops;let Statements;Function Parameters

/*
Patterns come in two forms: refutable and irrefutable. Patterns that will match for any possible value passed are irrefutable. 

Function parameters, let statements, and for loops can only accept irrefutable patterns, because the program cannot do anything meaningful when values don’t match. The if let and while let expressions accept refutable and irrefutable patterns, but the compiler warns against irrefutable patterns because by definition they’re intended to handle possible failure: the functionality of a conditional is in its ability to perform differently depending on success or failure.
*/

#[allow(warnings)]
#[test]
fn refutable_irrefutable() {
    let s_ir = Some(34);
    // error[E0005]: refutable pattern in local binding
    // let Some(x) = s_ir; // Some(x):pattern `None` not covered

    let s_ = Some(34);
    // Using if let and a block with refutable patterns instead of le
    if let Some(x) = s_ {
        println!("{}", x)
    }

    // warning: irrefutable `if let` pattern
    // this pattern will always match, so the `if let` is useless
    if let t_ = 5 {
        println!("{}", t_)
    }
    
    let t_ir = 5;
}

#[test]
fn t0() {
    // while let Conditional Loops

    let mut stack = vec![Some(10), Some(20), None, Some(30), Some(40)];

    // Similar in construction to if let, the while let conditional loop allows a while loop to run for as long as a pattern continues to match. 
    while let Some(Some(v)) = stack.pop() {
        println!("{}", v); // 遇到None变体时,循环自动结束
    } 

    // *****************************************************
    // for Loops

    let v = vec!['a', 'b', 'c'];
    /*
    In a for loop, the value that directly follows the keyword for is a pattern. For example, in for x in y the x is the pattern.
     */
    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }

    // *****************************************************
    // let Statements

    // x is a pattern that means “bind what matches here to the variable x.” 
    #[allow(warnings)]
    let x = 5;

    /*
    Rust compares the value (1, 2, 3) to the pattern (_x, _y, _z) and sees that the value matches the pattern, so Rust binds 1 to _x, 2 to _y, and 3 to _z. You can think of this tuple pattern as nesting three individual variable patterns inside it.
     */
    let (_x, _y, _z) = (1, 2, 3);

    // *****************************************************
    // Function Parameters(Function parameters can also be patterns.)

    // The values &(3, 5) match the pattern &(x, y), so x is the value 3 and y is the value 5.
    fn print_coordinates(&(x, y): &(i32, i32)) {
        println!("Current location: ({}, {})", x, y);
    }

    let point = (3, 5);
    print_coordinates(&point); // print->Current location: (3, 5)
}
