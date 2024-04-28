// Writing tests so they return a Result<T, E> enables you to use the question mark operator in the body of tests, which can be a convenient way to write tests that should fail if any operation within them returns an Err variant.
#[test]
fn t0() -> Result<(), String> {
    if 2 + 2 == 5 {
        Ok(())
    } else {
        Err(String::from("two plus two does not equal four")) // 测试失败
    }
}

fn divide(a: f32, b: f32) -> Result<f32, String> {
    if b == 0.0 {
        Err("Cannot divide by zero.".to_string())
    } else {
        Ok(a / b)
    }
}

#[test]
fn t1() -> Result<(), String> {
    let result = divide(10.0, 0.0)?;
    Ok(())
}

/*
You can’t use the #[should_panic] annotation on tests that use Result<T, E>. To assert that an operation returns an Err variant, don’t use the question mark operator on the Result<T, E> value. Instead, use assert!(value.is_err()).
*/
#[test]
fn t2() -> Result<(), String> {
    let result = divide(10.0, 0.0);
    assert!(result.is_err()); // 测试通过
    Ok(())
}
