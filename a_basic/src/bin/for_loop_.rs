// for循环

fn main() {
    /*
    A for expression is a syntactic construct for looping over elements provided by an implementation of std::iter::IntoIterator.
    If the iterator yields a value, that value is matched against the irrefutable pattern, the body of the loop is executed, and then control returns to the head of the for loop. If the iterator is empty, the for expression completes.
    */
    for i in 1..5 {
        println!("{i}")
    }

    for _ in 1..5 {
        println!("hello rust!")
    }

    let v = &["apples", "cake", "coffee"];

    for text in v {
        println!("I like {}.", text);
    }
}
