mod macro_assert_;

#[cfg(test)]
mod tests {

    
    #[test]
    fn main_test0() {
        assert_eq!(21, 3 * 7);
    }
}

fn main() {
    println!("Hello, world!");
}
