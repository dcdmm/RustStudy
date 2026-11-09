pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!(
                "猜测值必须大于等于1,得到: {}.",
                value
            );
        } else if value > 100 {
            panic!(
                "猜测值必须小于等于100,得到: {}.",
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
    Guess::new(10); // 测试失败(未按预期发生panic)
}

#[test]
/*
The should_panic attribute may optionally take an input string that must appear within the panic message. If the string is not found in the message, then the test will fail.
*/
#[should_panic(expected = "小于等于100")]
fn attribute_should_panic_test1() {
    Guess::new(200); // 测试通过(发生了panic,且panic信息包含expected子串"小于等于100")
}

#[test]
#[should_panic(expected = "xxxx")]
fn attribute_should_panic_test1_1() {
    Guess::new(200); // 测试失败(发生了panic,但panic信息不包含expected子串"xxxx")
}