// trait(Traits as Parameters/Returning Types that Implement Traits)

use std::fmt::Debug;
use std::fmt::Display;

#[allow(warnings)]
trait Summary {
    fn summarize(&self) -> String;
}

// This parameter accepts any type that implements the specified trait.
#[allow(warnings)]
fn notify_sample(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// 与上等价
// Trait Bound Syntax(语法糖)
#[allow(warnings)]
fn notify_bound<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

#[allow(warnings)]
fn notify_sample_long(item1: &impl Summary, item2: &impl Summary) {
    println!("Breaking news! {} {}", item1.summarize(), item2.summarize());
}

// 与上等价
#[allow(warnings)]
fn notify_bound_long<T: Summary>(item1: &T, item2: &T) {
    println!("Breaking news! {} {}", item1.summarize(), item2.summarize());
}

// Specifying Multiple Trait Bounds with the + Syntax
#[allow(warnings)]
fn notify_sample_many(_item: &(impl Summary + Display)) {}

// 与上等价
#[allow(warnings)]
fn notify_bound_many<T: Summary + Display>(_item: &T) {}

#[allow(warnings)]
fn some_function<T: Display + Clone, U: Clone + Debug>(_t: &T, _u: &U) -> i32 {
    3
}

// 与上等价
// 函数签名更加简洁
#[allow(warnings)]
fn some_function_where<T, U>(_t: &T, _u: &U) -> i32
// Clearer Trait Bounds with where Clauses
where
    T: Display + Clone,
    U: Clone + Debug,
{
    3
}


#[allow(warnings)]
struct Tweet {
    username: String,
    content: String,
    reply: bool,
    retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

// By using impl Summary for the return type, we specify that the returns_summarizable function returns some type that implements the Summary trait without naming the concrete type.
#[allow(warnings)]
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    }
}

fn main() {}