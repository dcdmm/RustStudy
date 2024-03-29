// for循环

/*
A for expression is a syntactic construct for looping over elements provided by an implementation of std::iter::IntoIterator.If the iterator yields a value, that value is matched against the irrefutable pattern, the body of the loop is executed, and then control returns to the head of the for loop.If the iterator is empty, the for expression completes.
 */

fn main() {
    let x = std::ops::Range {
        start: 'a',
        end: 'z',
    };
    for ss in x { // 支持遍历整数和字符类型的Range和RangeInclusive
        println!("{}", ss)
    }

    for i in 1..=5 {
        println!("{i}")
    }

    for _ in 1..5 {
        println!("hello rust!")
    }

    let mut v = ["apples", "cake", "coffee"];
    for text in v { // 迭代类型为&str;相等于执行于for text in v.into_iter() {
        println!("I like {}.", text);
    }
    for text in &v { // 迭代类型为&&str;相当于执行for text in &v.into_iter() {
        println!("I like {}.", text);
    }
    for text in &mut v { // 迭代类型为&mut &str;相当于执行for text in (&mut v).into_iter() {
        println!("I like {}.", text);
    }
}
