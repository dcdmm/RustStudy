// std::vec::Vec(参考C++ vector;参考 Java ArrayList;参考Python list)

/*
A contiguous growable array type with heap-allocated contents, written Vec<T>.

Vectors have O(1) indexing, amortized O(1) push (to the end) and O(1) pop (from the end).
 */

use std::thread::ScopedJoinHandle;

#[test]
fn t0() {
    let v0 = vec![1, 2, 3, 4];
    println!("{:?}", v0);

    let v1 = vec![0; 5];
    println!("{:?}", v1); // print->[0, 0, 0, 0, 0]

    let mut v3: Vec<i32> = Vec::new(); // 声明v3 Type annotations为Vec<i32>(编译器不能进行推断)
    println!("{:?}", v3); // print->[]

    let mut v31 = Vec::new();
    v31.push(3); // 推断v31的类型为Vec<i32>

    println!("{:?}", v3);
    let v4 = Vec::from([1, 2, 3, 4, 5]);
    println!("{:?}", v4); // print->[1, 2, 3, 4, 5]
}

#[test]
fn t1() {
    let v = vec![0, 2, 4, 6];
    println!("{}", v[1]); // 索引;print->2

    // panicked: index out of bounds: the len is 4 but the index is 5
    println!("{}", v[5]);
}

#[test]
fn t2() {
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{i}")
    }

    let mut v1 = vec![1, 2, 3, 4];
    for j in &mut v1 {
        *j *= -1;
    }
    println!("{:?}", v1)
}

// Using an Enum to Store Multiple Types
#[test]
fn multiple_types() {
    #[derive(Debug)]
    enum SpreadsheetCell {
        Int(i32),
        Float(f64),
        Text(String),
    }

    // If you don’t know the exhaustive set of types a program will get at runtime to store in a vector, the enum technique won’t work. Instead, you can use a trait object
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    for cell in row {
        match cell {
            SpreadsheetCell::Int(i) => println!("{}", i),
            SpreadsheetCell::Float(j) => println!("{}", j),
            SpreadsheetCell::Text(t) => println!("{}", t),
        }
    }
}
