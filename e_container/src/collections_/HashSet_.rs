// std::collections::HashSet(参考C++ unordered_set;参考 Java HashSet;参考Python set)

/*
A hash set implemented as a HashMap where the value is ().

As with the HashMap type, a HashSet requires that the elements implement the Eq and Hash traits. This can frequently be achieved by using #[derive(PartialEq, Eq, Hash)]. If you implement these yourself, it is important that the following property holds: 
`k1 == k2 -> hash(k1) == hash(k2)`

In other words, if two keys are equal, their hashes must be equal. Violating this property is a logic error.
*/
use std::collections::HashSet;

#[test]
fn t0() {
    let mut s0: HashSet<String> = HashSet::new();

    let mut s1 = HashSet::new();
    s1.insert("dmm"); // 推断s1的类型为HashSet<&str>

    let s2 = HashSet::from(["a", "b", "c"]);
    println!("{:?}", s2)
}
