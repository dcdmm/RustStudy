// 泛型trait

#[test]
fn t0() {
    trait Printer<T> {
        fn print(&self, value: T);
    }

    struct SimplePrinter;

    impl Printer<i32> for SimplePrinter {
        fn print(&self, value: i32) {
            println!("Integer: {}", value);
        }
    }

    impl Printer<f64> for SimplePrinter {
        fn print(&self, value: f64) {
            println!("Float: {}", value);
        }
    }
    
    let int_printer = SimplePrinter;
    int_printer.print(123);

    let float_printer = SimplePrinter;
    float_printer.print(3.14);
}

#[test]
fn t1() {
    use std::fmt::Display;
    use std::ops::Add;

    trait AddAndPrint<T>
    where
        T: Add<Output = T> + Display,
    {
        fn add_and_print<U>(&self, a: T, b: T) -> U
        where
            U: From<T>;
    }

    struct Operation;

    impl<T> AddAndPrint<T> for Operation
    where
        T: Add<Output = T> + Display,
    {
        fn add_and_print<U>(&self, a: T, b: T) -> U
        where
            U: From<T>,
        {
            let result = a + b;
            U::from(result)
        }
    }

    let op = Operation;
    let result_i32: i32 = op.add_and_print(10, 20);
    println!("{}", result_i32);
}

fn main() {}
