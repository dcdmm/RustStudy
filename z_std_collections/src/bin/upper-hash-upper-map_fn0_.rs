// std::collections::HashMap
/*
常见方法:
len / is_empty
get
insert
entry
remove
 */

fn main() {}


#[test]
fn len_is_empty_fn() {
    use std::collections::HashMap;

    let mut a = HashMap::new();
    assert_eq!(a.len(), 0);
    assert!(a.is_empty());
    a.insert(1, "a");
    assert_eq!(a.len(), 1);
    assert!(!a.is_empty());
}

#[test]
fn get_fn() {
    use std::collections::HashMap;

    let mut map = HashMap::new();
    map.insert(1, "a");
    /*
    Returns a reference to the value corresponding to the key.

    The key may be any borrowed form of the map’s key type, but Hash and Eq on the borrowed form must match those for the key type.
     */
    assert_eq!(map.get(&1), Some(&"a"));
    assert_eq!(map.get(&2), None);
}

#[test]
fn insert_fn() {
    use std::collections::HashMap;

    let mut map = HashMap::new();
    /*
    Inserts a key-value pair into the map.

    If the map did not have this key present, None is returned.

    If the map did have this key present, the value is updated, and the old value is returned.
     */
    assert_eq!(map.insert(37, "a"), None);
    assert_eq!(map.is_empty(), false);

    map.insert(37, "b");
    assert_eq!(map.insert(37, "c"), Some("b"));
    assert_eq!(map[&37], "c");
}

#[test]
fn entry_fn() {
    use std::collections::HashMap;

    let mut letters = HashMap::new();
    // Gets the given key’s corresponding entry in the map for in-place manipulation.
    for ch in "a short treatise on fungi".chars() {
        letters.entry(ch).and_modify(|counter| *counter += 1).or_insert(1);
    }

    assert_eq!(letters[&'s'], 2);
    assert_eq!(letters[&'t'], 3);
    assert_eq!(letters[&'u'], 1);
    assert_eq!(letters.get(&'y'), None);
}

#[test]
fn remove_fn() {
    use std::collections::HashMap;

    let mut map = HashMap::new();
    map.insert(1, "a");
    /*
    Removes a key from the map, returning the value at the key if the key was previously in the map.

    The key may be any borrowed form of the map’s key type, but Hash and Eq on the borrowed form must match those for the key type.
     */
    assert_eq!(map.remove(&1), Some("a"));
    assert_eq!(map.remove(&1), None);
}