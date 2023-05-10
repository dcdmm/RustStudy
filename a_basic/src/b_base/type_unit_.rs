// ()(Primitive Type unit)

/*
The () type, also called “unit”.

The () type has exactly one value (), and is used when there is no other meaningful value that could be returned. 
*/

// fn long() -> () {}

// fn long() {} // 与上等价,可以省略`-> ()`

fn returns_i64() -> i64 {
    1i64
}

fn returns_unit() {
    // The semicolon `;` can be used to discard the result of an expression at the end of a block, making the expression (and thus the block) evaluate to ().
    1i64;
}

fn main() {
    let _is_i64 = returns_i64();
    let _is_unit = returns_unit();
}