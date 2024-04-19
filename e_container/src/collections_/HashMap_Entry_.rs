// Enum std::collections::hash_map::Entry

use std::collections::HashMap;

#[test]
fn t0() {
    let mut map: HashMap<&str, u32> = HashMap::new();
    let map_entry = map.entry("hello java");
    println!("{}", map_entry.key()); // print->hello java
}

#[test]
fn t1() {
    
}

#[test]
fn t2() {
    let mut map: HashMap<&str, u32> = HashMap::new();
    map.entry("poneyland").and_modify(|e| *e += 2).or_insert(42);
    println!("{:?}", map);
    map.entry("poneyland").and_modify(|e| *e += 1).or_insert(42);
    println!("{:?}", map);
}
