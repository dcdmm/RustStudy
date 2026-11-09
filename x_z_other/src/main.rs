// 影响整个crate(即main.rs或lib.rs代码前添加#![attribute])
#![allow(warnings)]

mod attributes;
mod std_functions;
mod std_structs_enums;
mod std_traits;

fn main() {
    println!("Hello, world!");
}
