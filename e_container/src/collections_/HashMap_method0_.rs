// std::collections::HashMap

/*
常见方法:
len
is_empty

keys / values

clear

get
entry

insert

remove
 */

use std::collections::HashMap;

#[test]
fn len_is_empty_test() {
    let mut a = HashMap::new();
    assert_eq!(a.len(), 0);
    assert!(a.is_empty());
    a.insert(1, "a");
    assert_eq!(a.len(), 1);
    assert!(!a.is_empty());
}

#[test]
fn keys_values_test() {
    let map = HashMap::from([("a", 1), ("b", 2), ("c", 3)]);
    for key in map.keys() {
        println!("{key}");
    }
    for value in map.values() {
        println!("{value}");
    }
}

#[test]
fn clear_test() {
    let mut map = HashMap::from([("a", 1), ("b", 2), ("c", 3)]);
    map.clear();

    println!("{:?}", map);
    println!("{:?}", map.len()); // print->0
    println!("{:?}", map.is_empty()); // print->true
}

#[test]
fn get_test() {
    let mut map = HashMap::from([("a", 1), ("b", 2), ("c", 3)]);

    /*
    Returns a reference to the value corresponding to the key.

    The key may be any borrowed form of the map’s key type, but Hash and Eq on the borrowed form must match those for the key type.
     */
    assert_eq!(map.get(&"a"), Some(&1));
    assert_eq!(map.get(&"d"), None);
}

#[test]
fn insert_test() {
    let mut map = HashMap::new();
    /*
    Inserts a key-value pair into the map.

    If the map did not have this key present, None is returned.

    If the map did have this key present, the value is updated, and the old value is returned.
     */
    assert_eq!(map.insert(37, "a"), None);
    println!("{:?}", map);

    map.insert(37, "b");
    assert_eq!(map.insert(37, "c"), Some("b"));
    assert_eq!(map[&37], "c");
}

#[test]
fn entry_test() {
    let mut letters = HashMap::new();
    let c = letters.entry('c');
    // Gets the given key’s corresponding entry in the map for in-place manipulation.
    for ch in "dcdmm".chars() {
        letters
            .entry(ch)
            .and_modify(|counter| *counter += 1)
            .or_insert(1);
    }

    /*
    letters结果解析:
    ch='d' ===> 执行or_insert(1) ===> {'d': 1}
    ch='c' ===> 执行or_insert(1) ===> {'d': 1, 'c': 1}
    ch='d' ===> 执行and_modify(|counter| *counter += 1) ===> {'d': 2, 'c': 1}
    ch='m' ===> 执行or_insert(1) ===> {'d': 2, 'c': 1, 'm': 1}
    ch='m' ===> 执行and_modify(|counter| *counter += 1) ===> {'d': 2, 'c': 1, 'm': 2}
     */
    println!("{:?}", letters);
}

#[test]
fn remove_test() {
    let mut map = HashMap::from([(1, "a"), (2, "b")]);

    /*
    Removes a key from the map, returning the value at the key if the key was previously in the map.

    The key may be any borrowed form of the map’s key type, but Hash and Eq on the borrowed form must match those for the key type.
     */
    assert_eq!(map.remove(&1), Some("a"));
    println!("{:?}", map); // print->{2: "b"}
    assert_eq!(map.remove(&1), None);
    println!("{:?}", map); // print->{2: "b"}
}
