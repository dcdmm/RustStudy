use i_macro::{export_, use0_};

fn main() {
    export_!();
    println!("use0_!(3, 5) = {}", use0_!(3, 5));
}

#[test]
fn t0() {
}
