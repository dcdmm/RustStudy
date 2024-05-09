#[test]
fn t0() {
    macro_rules! base0 {
        () => {
            1 + 3
        };
    }
    let a = base0!();
    let b = base0! {};
    let c = base0![];
}

fn main() {}
