// std::collections::HashMap(参考C++ unordered_map;参考 Java HashMap;参考Python dict)

/*
Just like vectors, hash maps store their data on the heap.

Like vectors, hash maps are homogeneous: all of the keys must have the same type as each other, and all of the values must have the same type.
 */

use std::collections::HashMap;

#[test]
fn t0() {
    let hm: HashMap<String, String> = HashMap::new();

    let mut hm = HashMap::new();
    hm.insert("Adventures of Huckleberry Finn", "My favorite book."); // 推断hm的类型为HashMap<&str, &str>

    let solar_distance = HashMap::from([
        ("Mercury", 0.4),
        ("Venus", 0.7),
        ("Earth", 1.0),
        ("Mars", 1.5),
    ]);
    println!("{:?}", solar_distance)
}

#[test]
fn t1() {
    let map = HashMap::from([("a", 1), ("b", 2), ("c", 3)]);
    println!("{}", map["a"]); // print->1
    // no entry found for key
    println!("{}", map["d"]);
    
}