// trait(Traits as Parameters/Trait Bound Syntax)

use std::fmt::Debug;
use std::fmt::Display;

#[allow(warnings)]
trait Summary {
    fn summarize(&self) -> String;
}

// This parameter accepts any type that implements the specified trait.
// The impl Trait syntax works for straightforward cases but is actually syntax sugar for a longer form known as a trait bound
#[allow(warnings)]
fn notify_sample(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// 与上等价
// Trait Bound Syntax
// 函数定义:fn func_<T: tbs>(x: T)
// 结构体定义:struct Struct_<T: tbs>
// 枚举定义:enum Enum_<T: tbs>
// 方法定义:impl<T: tbs> Type_<T>
// 实现特质(为具体的泛型类型Type_<T>(其泛型参数T满足tbs约束)实现特质Trait_):impl<T: tbs> Trait_ for Type_<T>
// 实现特质(为满足tbs约束的泛型类型T实现特质Trait_):impl<T: tbs> Trait_ for T
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

fn main() {}
