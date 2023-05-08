// Block expressions

/*
The syntax for a block is {, then any inner attributes, then any number of statements, then an optional expression, called the final operand, and finally a }.

Statements are usually required to be followed by a semicolon, with two exceptions:
1. Item declaration statements do not need to be followed by a semicolon.
2. Expression statements usually require a following semicolon except if its outer expression is a flow control expression.

When evaluating a block expression, each statement, except for item declaration statements, is executed sequentially. Then the final operand is executed, if given.

The type of a block is the type of the final operand, or () if the final operand is omitted.
 */

fn fn_call() {}

fn main() {
    // Blocks are always value expressions and evaluate the last operand in value expression context.

    let _: () = {
        fn_call();
    };

    let five: i32 = {
        fn_call();
        5
    };

    assert_eq!(5, five);
}