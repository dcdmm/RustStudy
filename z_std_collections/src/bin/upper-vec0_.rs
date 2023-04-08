// std::vec::Vec

/*
A contiguous growable array type with heap-allocated contents, written Vec<T>.

Vectors have O(1) indexing, amortized O(1) push (to the end) and O(1) pop (from the end).

Vectors ensure they never allocate more than isize::MAX bytes.
 */

fn main() {
    let v = vec![1, 2, 3, 4];
    println!("{}", v[3]); // 通过索引访问Vec中的元素

    // panic:thread 'main' panicked at 'index out of bounds: the len is 4 but the index is 4'
    println!("{}", v[4])
}


#[test]
fn create() {
    // Constructs a new, empty Vec<T>.
    // The vector will not allocate until elements are pushed onto it.
    let mut vec = Vec::new(); // The numbers we place inside are all of type i32, and Rust infers this from the data, so we don’t need the Vec<i32> annotation.
    vec.push(1);
    vec.push(2);
    assert_eq!(vec.len(), 2);
    assert_eq!(vec[0], 1);

    // The vec! macro is provided for convenient initialization:
    let vec1 = vec![1, 2];
    assert_eq!(vec1.len(), 2);
    assert_eq!(vec1[0], 1);
}


#[test]
fn iterating() {
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{i}")
    }

    let mut v1 = vec![1, 2, 3, 4];
    for j in &mut v1 {
        // We can also iterate over mutable references to each element in a mutable vector in order to make changes to all the elements.
        *j += 1;
    }

    for k in &mut v1 {
        println!("{k}")
    }
}

#[test]
fn multiple_types() {
    /*
    Vectors can only store values that are the same type. This can be inconvenient;
    there are definitely use cases for needing to store a list of items of different types.
    Fortunately, the variants of an enum are defined under the same enum type, so when we need one type to represent elements of different types, we can define and use an enum!
     */
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let _row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    /*
    Rust needs to know what types will be in the vector at compile time so it knows exactly how much memory on the heap will be needed to store each element.
    We must also be explicit about what types are allowed in this vector.
    If Rust allowed a vector to hold any type, there would be a chance that one or more of the types would cause errors with the operations performed on the elements of the vector.
    Using an enum plus a match expression means that Rust will ensure at compile time that every possible case is handled

    If you don’t know the exhaustive set of types a program will get at runtime to store in a vector, the enum technique won’t work. Instead, you can use a trait objec
     */
}
