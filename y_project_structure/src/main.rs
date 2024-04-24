#![allow(warnings)]

// For a module named `file_path` declared in the crate root, the compiler will look for the module’s code in:
// 方式一(见:style1): src/file_path.rs
// 方式二(见:style2): src/file_path/mod.rs
//
// For a module named `file_path_sub` that is a submodule of `file_path`, the compiler will look for the module’s code in:
// 方式一: src/file_path/file_path_sub.rs
// 方式二: src/file_path/file_path_sub/mod.rs
//
// 注意:方式一、方式二不能混用

mod base0;
mod base1;

mod public_structs_enum_;

mod style1;
mod style2;

// mod use_keyword0_ {
//     // use_keyword0_.rs代码
// }
mod use_keyword0_; // 与上等价
mod use_keyword1_;
mod use_keyword2_;

#[test]
fn t0() {
    use style1::arithmetic0_::sum_sub::sum;
    // error[E0603]: function `sub` is private
    // use style1::arithmetic0_::sum_sub::sub;

    // error[E0603]: module `mul_div` is private
    // use style2::arithmetic1_::mul_div;

    // error[E0603]: module `arithmetic2_` is private
    // use style2::arithmetic2_;
}

fn main() {
    use base0::eat_at_restaurant as eat_at_restaurant_;
    use use_keyword0_::customer::eat_at_restaurant;

    eat_at_restaurant_();
    eat_at_restaurant();

    use use_keyword2_::hosting;
}
