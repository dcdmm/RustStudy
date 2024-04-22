// std::collections::HashMap(参考C++ unordered_map;参考 Java HashMap;参考Python dict)

/*
A hash map implemented with quadratic probing and SIMD lookup.

It is required that the keys implement the Eq and Hash traits, although this can frequently be achieved by using #[derive(PartialEq, Eq, Hash)]. If you implement these yourself, it is important that the following property holds:
`k1 == k2 -> hash(k1) == hash(k2)`

In other words, if two keys are equal, their hashes must be equal. Violating this property is a logic error.
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
