// 元组(Primitive Type tuple)
// 类似c++ std::tuple

/*
A tuple is a general way of grouping together a number of values with a variety of types into one compound type. Tuples have a fixed length: once declared, they cannot grow or shrink in size.
*/

fn main() {
    let tup = (500, 5.4, 1);

    let (x, y, z) = tup;
    println!("The value of x, y, z is: {x}, {y}, {z}");

    // Tuples are a sequence. This means that they can be accessed by position
    println!("{}", tup.0);
    println!("{}", tup.1);
    println!("{}", tup.2);

    let mut tup1 = (1, 2, 3.0);
    tup1.0 = -1;
    println!("{}", tup1.0)
}
