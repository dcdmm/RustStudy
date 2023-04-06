// 模式匹配(while let Conditional Loops;for Loops;let Statements;Function Parameters)

fn main() {
    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);

    // Similar in construction to if let
    while let Some(top) = stack.pop() {
        println!("{}", top);
    }

    // *****************************************************
    let v = vec!['a', 'b', 'c'];
    /*
    In a for loop, the value that directly follows the keyword for is a pattern.
    For example, in for x in y the x is the pattern.
     */
    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }

    // *****************************************************
    /*
    Every time you've used a let statement like this you've been using patterns, although you might not have realized it! More formally, a let statement looks like this:
    let PATTERN = EXPRESSION;

    In statements like let x = 5; with a variable name in the PATTERN slot, the variable name is just a particularly simple form of a pattern.
    Rust compares the expression against the pattern and assigns any names it finds.
    So in the let x = 5; example, x is a pattern that means “bind what matches here to the variable x.” Because the name x is the whole pattern, this pattern effectively means “bind everything to the variable x, whatever the value is.”
     */
    let _a = 5;

    /*
    Here, we match a tuple against a pattern.
    Rust compares the value (1, 2, 3) to the pattern (x, y, z) and sees that the value matches the pattern, so Rust binds 1 to x, 2 to y, and 3 to z.
    You can think of this tuple pattern as nesting three individual variable patterns inside it.

    If the number of elements in the pattern doesn’t match the number of elements in the tuple, the overall type won’t match and we’ll get a compiler error.
     */
    let (_x, _y, _z) = (1, 2, 3);

    /*
    Function parameters can also be patterns.
    The x part is a pattern! As we did with let, we could match a tuple in a function’s arguments to the pattern.

    This code prints Current location: (3, 5). The values &(3, 5) match the pattern &(x, y), so x is the value 3 and y is the value 5.
     */
    fn print_coordinates(&(x, y): &(i32, i32)) {
        println!("Current location: ({}, {})", x, y);
    }

    let point = (3, 5);
    print_coordinates(&point);
}
