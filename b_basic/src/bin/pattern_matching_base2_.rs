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
    // In a for loop, the value that directly follows the keyword for is a pattern.
    for (index, value) in v.iter().enumerate() {
        println!("{} is at index {}", value, index);
    }

    // *****************************************************
    // Every time you've used a let statement like this you've been using patterns, although you might not have realized it! More formally, a let statement looks like this:
    // let PATTERN = EXPRESSION;
    let _a = 5;

    // If the number of elements in the pattern doesn’t match the number of elements in the tuple, the overall type won’t match and we’ll get a compiler error.
    let (_x, _y, _z) = (1, 2, 3);

    // *****************************************************
    // Function parameters can also be patterns.
    // The x part is a pattern! As we did with let, we could match a tuple in a function’s arguments to the pattern.
    fn print_coordinates(&(x, y): &(i32, i32)) {
        println!("Current location: ({}, {})", x, y);
    }

    let point = (3, 5);
    print_coordinates(&point);
}
