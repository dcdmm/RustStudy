// std::collections::HashMap

/*
Just like vectors, hash maps store their data on the heap.

Like vectors, hash maps are homogeneous: all of the keys must have the same type as each other, and all of the values must have the same type.
 */

use std::collections::HashMap;

#[test]
fn create() {
    /*
    Creates an empty HashMap.
    The hash map is initially created with a capacity of 0, so it will not allocate until it is first inserted into.
     */
    let _mut_scores: HashMap<String, String> = HashMap::new();
}

#[test]
fn iterating() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    for (key, value) in &scores {
        println!("{key}: {value}");
    }
}