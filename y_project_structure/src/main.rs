#![allow(warnings)]

// For a module named `file_path` declared in the crate root, the compiler will look for the module’s code in:
// 方式一(如:file_path_style1): src/file_path.rs
// 方式二(如:file_path_style2): src/file_path/mod.rs
//
// For a module named `file_path_sub` that is a submodule of `file_path`, the compiler will look for the module’s code in:
// 方式一: src/file_path/file_path_sub.rs
// 方式二: src/file_path/file_path_sub/mod.rs
//
// 注意:方式一、方式二不能混用

mod base0;
mod base1;
mod base2_public_structs_enum_;

mod file_path_style1;
mod file_path_style2;

// mod use_keyword0_ {
//     // use_keyword0_.rs代码
// }
mod keyword_use0_; // 与上等价
mod keyword_use1_;
mod keyword_use2_;

fn main() {
    use file_path_style1::arithmetic0_::sum_sub::sum;
    // error[E0603]: function `sub` is private
    // use style1::arithmetic0_::sum_sub::sub;

    // error[E0603]: module `mul_div` is private
    // use style2::arithmetic1_::mul_div;

    // error[E0603]: module `arithmetic2_` is private
    // use style2::arithmetic2_;
}
