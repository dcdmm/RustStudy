#[cfg(test)]
mod tests_0 {
    #[test]
    fn main_test0_t0() {
        assert!(true)
    }

    #[test]
    fn main_test0_t1() {
        assert_eq!(3, 1 + 2)
    }
}

#[cfg(test)]
mod tests_1 {
    #[test]
    fn main_tests1_t0() {
        assert!(true)
    }

    #[test]
    fn main_test1_t1() {
        assert_eq!(3, 1 + 2)
    }
}

#[test]
fn t0() {
    assert_eq!(3, 1 + 2)
}

fn main() {}
