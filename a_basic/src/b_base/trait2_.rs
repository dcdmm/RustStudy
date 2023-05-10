// trait(Traits as Parameters)

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

// 与上等价
// Trait Bound Syntax(语法糖)
#[allow(dead_code)]
fn notify_bound<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

#[allow(dead_code)]
fn notify_sample_long(item1: &impl Summary, item2: &impl Summary) {
    println!("Breaking news! {} {}", item1.summarize(), item2.summarize());
}

// 与上等价
#[allow(dead_code)]
fn notify_bound_long<T: Summary>(item1: &T, item2: &T) {
    println!("Breaking news! {} {}", item1.summarize(), item2.summarize());
}

// Specifying Multiple Trait Bounds with the + Syntax
#[allow(dead_code)]
fn notify_sample_many(_item: &(impl Summary + Display)) {}

// 与上等价
#[allow(dead_code)]
fn notify_bound_many<T: Summary + Display>(_item: &T) {}

#[allow(dead_code)]
fn some_function<T: Display + Clone, U: Clone + Debug>(_t: &T, _u: &U) -> i32 {
    3
}

// 与上等价
#[allow(dead_code)]
// 函数签名更加简洁
fn some_function_where<T, U>(_t: &T, _u: &U) -> i32
// Clearer Trait Bounds with where Clauses
    where
        T: Display + Clone,
        U: Clone + Debug,
{
    3
}

fn main() {}
