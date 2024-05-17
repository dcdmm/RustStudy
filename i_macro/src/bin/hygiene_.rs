// By default, all identifiers referred to in a macro are expanded as-is, and are looked up at the macro's invocation site. This can lead to issues if a macro refers to an item or macro which isn't in scope at the invocation site.

#[allow(warnings)]
#[test]
fn error() {
    macro_rules! using_a {
        ($e:expr) => {{
            let a = 42;
            $e
        }};
    }

    /*
    展开宏调用,即:
    let four = {
        let a = 42;
        a / 10
    }; // let a = 42;和a / 10位于不同上下文中
     */
    // error[E0425]: cannot find value `a` in this scope
    // let four = using_a!(a / 10);
}

#[test]
fn right() {
    macro_rules! using_a {
        ($a:ident, $e:expr) => {{
            let $a = 42;
            $e
        }};
    }
    /*
    展开宏调用,即:
    let four = {
        let a = 42;
        a / 10
    }; // let a = 42;和a / 10位于相同上下文中
     */
    let four = using_a!(a, a / 10);
    println!("{}", four);
}
fn main() {}
