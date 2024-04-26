#![allow(warnings)]

mod base0_;
mod base2_public_structs_enum_;

mod file_path_style1;
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
    // use style1::arithmetic0_::sum_sub::sub;

    // error[E0603]: module `mul_div` is private
    // use style2::arithmetic1_::mul_div;

    // error[E0603]: module `arithmetic2_` is private
    // use style2::arithmetic2_;
}
