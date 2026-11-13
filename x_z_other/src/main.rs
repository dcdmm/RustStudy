// 在crate根文件(main.rs或lib.rs)开头添加#![attribute],作用于以该文件为根的整个crate
#![allow(non_snake_case)]

mod attributes;
mod cargo_feature_test;
#[allow(warnings)]
mod std_functions;
#[allow(warnings)]
mod std_structs_enums;
#[allow(warnings)]
mod std_traits;

fn main() {
    println!("Hello, world!");
}
