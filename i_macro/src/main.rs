use i_macro::export_;
use i_macro::*; // 使用通配符引入所有宏

fn main() {
    export_!();

    let _ = use0_!(3, 5);
    let _ = use1_!(3, 5);
    let _ = use2_!(3, 5);
    let _ = use3_!(3, 5);
}
