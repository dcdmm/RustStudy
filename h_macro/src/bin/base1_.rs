#[test]
fn error() {
    macro_rules! using_a {
        ($e:expr) => {{
            let a = 42;
            $e
        }};
    }

    let four = using_a!(a / 10);
}

#[test]
fn right() {
    macro_rules! using_a {
        ($a:ident, $e:expr) => {{
            let $a = 42;
            $e
        }};
    }
    let four = using_a!(a, a / 10);
}

fn main() {}
