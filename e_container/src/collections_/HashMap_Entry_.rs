// Enum std::collections::hash_map::Entry

// A view into a single entry in a map, which may either be vacant or occupied.

use std::collections::HashMap;

#[test]
fn key_test() {
    let mut map: HashMap<&str, u32> = HashMap::new();
    let map_entry = map.entry("hello rust");

    // Returns a reference to this entry’s key.
    println!("{}", map_entry.key()); // print->hello rust
}

#[test]
fn or_insert_test() {
    let mut map: HashMap<&str, u32> = HashMap::new();

    // Ensures a value is in the entry by inserting the default if empty, and returns a mutable reference to the value in the entry.
    map.entry("xiaobai").or_insert(3);
    println!("{:?}", map); // print->{"xiaobai": 3}

    map.entry("xiaobai").or_insert(10);
    println!("{:?}", map); // print->{"xiaobai": 3}
    let oi = map.entry("xiaobai").or_insert(10);
    *oi *= 2;
    println!("{:?}", map); // print->{"xiaobai": 6}

    *map.entry("dcdmm").or_insert(1) *= 100;
    println!("{:?}", map); // print->{"xiaobai": 6, "dcdmm": 100}
}

#[test]
fn and_modify_test() {
    let mut map: HashMap<&str, u32> = HashMap::new();

    map.entry("xiaobai").or_insert(1);
    println!("{:?}", map); // print->{"xiaobai": 1}

    // Provides in-place mutable access to an occupied entry before any potential inserts into the map.
    map.entry("xiaobai").and_modify(|e| *e += 1);
    println!("{:?}", map); // print->{"xiaobai": 2}

    let am = map.entry("poneyland").and_modify(|e| *e += 1);
    am.or_insert(42);
    println!("{:?}", map); // print->{"xiaobai": 2, "poneyland": 42}
    
    map.entry("poneyland").and_modify(|e| *e += 1).or_insert(42);
    println!("{:?}", map); // print->{"xiaobai": 2, "poneyland": 43}
}
