pub struct Guess {
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

#[test]
// The should_panic attribute makes the test only pass if it actually panics.
#[should_panic]
fn attribute_should_panic_test0() {
    Guess::new(200); // 测试通过
}

#[test]
#[should_panic]
fn attribute_should_panic_test0_1() {
    // note: test did not panic as expected
    Guess::new(10); // 测试失败
}

#[test]
/*
The should_panic attribute may optionally take an input string that must appear within the panic message. If the string is not found in the message, then the test will fail.
*/
#[should_panic(expected = "less than or equal to 100")]
fn attribute_should_panic_test1() {
    Guess::new(200); // 测试通过
}

#[test]
#[should_panic(expected = "xxxx")]
fn attribute_should_panic_test1_1() {
    /*
    note: panic did not contain expected string
        panic message: `"Guess value must be less than or equal to 100, got 200."`,  
    expected substring: `"xxxx"`
     */
    Guess::new(200); // 测试失败
}