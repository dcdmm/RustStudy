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

fn main() {
    use file_path_style1::arithmetic0_::sum_sub::sum;
    // error[E0603]: function `sub` is private
    // use file_path_style1::arithmetic0_::sum_sub::sub;

    // error[E0603]: module `mul_div` is private
    // use file_path_style2::arithmetic21_::mul_div;

    // error[E0603]: module `arithmetic22_` is private
    // use file_path_style2::arithmetic22_;
}
