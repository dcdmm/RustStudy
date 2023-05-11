// 元组(Primitive Type tuple)

/*
A tuple is a general way of grouping together a number of values with a variety of types into one compound type.
Tuples have a fixed length: once declared, they cannot grow or shrink in size.
*/

#[test]
fn t0() {
    let tup = (500, 5.4, 1);

    let (x, y, z) = tup;
    println!("The value of x, y, z is: {x}, {y}, {z}");

    // Tuples are a sequence. This means that they can be accessed by position
    println!("{}", tup.0);
    println!("{}", tup.1);
    println!("{}", tup.2);
}
