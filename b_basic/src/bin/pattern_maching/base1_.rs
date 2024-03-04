// while let Conditional Loops;for Loops;let Statements;Function Parameters

#[test]
fn t0() {
    // while let Conditional Loops

    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);

    // Similar in construction to if let, the while let conditional loop allows a while loop to run for as long as a pattern continues to match. 
    while let Some(top) = stack.pop() {
        println!("{}", top);
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
