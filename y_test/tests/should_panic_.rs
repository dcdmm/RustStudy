#[cfg(test)]
mod tests {
    // The should_panic attribute makes the test only pass if it actually panics.
    #[test]
    // #[should_panic]
    // To make should_panic tests more precise, we can add an optional expected parameter to the should_panic attribute. The test harness will make sure that the failure message contains the provided text.
    #[should_panic(expected = "less than or equal to 100")]
    fn greater_than_100() {
        pub struct Guess {
            #[allow(dead_code)]
            value: i32,
        }
        
        impl Guess {
            pub fn new(value: i32) -> Guess {
                if value < 1 {
                    panic!(
                        "Guess value must be greater than or equal to 1, got {}.",
                        value
                    );
                } else if value > 100 {
                    panic!(
                        "Guess value must be less than or equal to 100, got {}.",
                        value
                    );
                }
        
                Guess { value }
            }
        }

        Guess::new(200);
    }
}
