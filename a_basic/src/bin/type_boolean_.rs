// 布尔类型与逻辑运算

// The boolean type or bool is a primitive data type that can take on one of two values, called true and false.

/*
Logical not
b	    !b
true	false
false	true

Logical or
a	    b	    a | b
true	true	true
true	false	true
false	true	true
false	false	false

Logical and
a	    b	    a & b
true	true	true
true	false	false
false	true	false
false	false	false

Logical xor
a	    b	    a ^ b
true	true	false
true	false	true
false	true	true
false	false	false

Comparisons
a	    b	    a == b
true	true	true
true	false	false
false	true	false
false	false	true

Comparisons
a	    b	    a > b
true	true	false
true	false	true
false	true	false
false	false	false
*/
fn main() {
    let _x = true;
    let y = false;
    println!("{}", y)
}