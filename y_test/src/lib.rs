#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}


impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod tests {
    // use crate::*;
    use super::*; // 与上等价

    #[test]
    fn another() {
        /*
         Tests fail when something in the test function panics. 
         Each test is run in a new thread, and when the main thread sees that a test thread has died, the test is marked as failed.
         */
        panic!("Make this test fail");
    }

    #[test]
    fn macro_std_assert0() {
        let larger = Rectangle {
            width: 8,
            height: 7,
        };
        let smaller = Rectangle {
            width: 5,
            height: 1,
        };
        
        /*
        Asserts that a boolean expression is true at runtime.

        This will invoke the panic! macro if the provided expression cannot be evaluated to true at runtime.
         */
        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn macro_std_assert1() {
        // assert with a custom message
        let a = 3; let b = 27;
        /*
        Custom Messages
        This macro has a second form, where a custom panic message can be provided with or without arguments for formatting. 
        See std::fmt for syntax for this form. 
        Expressions used as format arguments will only be evaluated if the assertion fails.
         */
        assert!(a + b == 31, "a 等于: {}, b 等于: {}", a, b);
    }

    #[test]
    fn macro_std_assert_eq0() {
        /*    
        Asserts that two expressions are equal to each other (using PartialEq).

        On panic, this macro will print the values of the expressions with their debug representations.

        Like assert!, this macro has a second form, where a custom panic message can be provided.
         */
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn macro_std_assert_eq1() {
        let a = 3;
        let b = 1 + 5;

        assert_eq!(a, b, "we are testing addition with {} and {}", a, b);
    }


    #[test]
    fn macro_std_assert_ne() {
        /*    
        Asserts that two expressions are not equal to each other (using PartialEq).

        On panic, this macro will print the values of the expressions with their debug representations.

        Like assert!, this macro has a second form, where a custom panic message can be provided.
         */
        assert_ne!(4, add_two(2));
    }
}