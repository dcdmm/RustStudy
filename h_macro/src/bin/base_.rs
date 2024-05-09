use std::vec;

#[test]
fn t0() {


    macro_rules! one_expression {
        ($e:expr) => {...};
    }

    macro_rules! times_five {
        ($e:expr) => { 5 * $e };
    }

    macro_rules! multiply_add {
        ($a:expr, $b:expr, $c:expr) => { $a * ($b + $c) };
    }

    macro_rules! discard {
        ($e:expr) => {};
    }
    macro_rules! repeat {
        ($e:expr) => { $e; $e; $e; };
    }

}

fn main() {
    let a = vec!(1, 2);
    println!("{:?}", a);
    let b = vec![1, 2, 3];
}
