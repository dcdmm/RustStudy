#![allow(warnings)]

mod base0_;
pub mod base1_;
mod base2_public_structs_enum_;

mod file_path_style1;
use file_path_style1::file_path_style1_sub::arithmetic11_::mul_div::add;
mod file_path_style2;
use file_path_style2::file_path_style2_sub::arithmetic3_::primary::pt;

// mod use_keyword0_ {
//     // use_keyword0_.rs代码
// }
mod keyword_use0_; // 与上等价
mod keyword_use1_;
mod keyword_use2_;

// x_package_crate_module为库名(根文件为src/lib.rs),使用lib.rs中的内容:
// 1. lib crate内部(lib.rs及其mod引入的文件): use crate::xxx
// 2. 同一package中的其他crate(如main.rs、src/bin/*.rs、tests/*.rs): use x_package_crate_module::xxx,无需mod,也无需在[dependencies]中声明
use x_package_crate_module::hello_from_lib;

fn main() {
    hello_from_lib(); // print->hello from lib.rs

    use file_path_style1::arithmetic0_::sum_sub::sum;
    // error[E0603]: function `sub` is private
    // use file_path_style1::arithmetic0_::sum_sub::sub;

    // error[E0603]: module `mul_div` is private
    // use file_path_style2::arithmetic21_::mul_div;

    // error[E0603]: module `arithmetic22_` is private
    // use file_path_style2::arithmetic22_;
}
