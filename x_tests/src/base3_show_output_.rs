// Showing Function Output

fn prints_and_returns_10(a: i32) -> i32 {
    println!("得到值: {}", a);
    10
}

// --show-output: Displays the stdout and stderr of successful tests after all tests have run.

// x_tests目录下执行: cargo test --bin x_tests base3_show_output_ -- --show-output

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn this_test_will_pass() {
        let value = prints_and_returns_10(4);
        assert_eq!(10, value); // 测试通过(println!输出仅在加--show-output时显示)
    }

    #[test]
    fn this_test_will_fail() {
        let value = prints_and_returns_10(8);
        assert_eq!(5, value); // 测试失败(println!输出始终显示)
    }
}
