// 特征(Traits as Parameters)

use std::fmt::Debug;
use std::fmt::Display;

#[allow(dead_code)]
trait Summary {
    fn summarize(&self) -> String;
}

// This parameter accepts any type that implements the specified trait.
#[allow(dead_code)]
fn notify_sample(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// Trait Bound Syntax
#[allow(dead_code)]
fn notify_bound<T: Summary>(item: &T) {
    // 与上等价
    println!("Breaking news! {}", item.summarize());
}

#[allow(dead_code)]
fn notify_sample_long(item1: &impl Summary, item2: &impl Summary) {
    println!("Breaking news! {} {}", item1.summarize(), item2.summarize());
}

#[allow(dead_code)]
fn notify_bound_long<T: Summary>(item1: &T, item2: &T) {
    // 与上等价
    println!("Breaking news! {} {}", item1.summarize(), item2.summarize());
}

// Specifying Multiple Trait Bounds with the + Syntax
#[allow(dead_code)]
fn notify_sample_many(_item: &(impl Summary + Display)) {}

#[allow(dead_code)]
fn notify_bound_many<T: Summary + Display>(_item: &T) {} // 与上等价

#[allow(dead_code)]
fn some_function<T: Display + Clone, U: Clone + Debug>(_t: &T, _u: &U) -> i32 {
    3
}

// Clearer Trait Bounds with where Clauses
#[allow(dead_code)]
fn some_function_where<T, U>(_t: &T, _u: &U) -> i32
// 函数签名更加简洁
where
    T: Display + Clone,
    U: Clone + Debug,
{
    3
} // 与上等价

fn main() {}
