// Trait std::ops::Deref/Implicit Deref Coercions with Functions and Methods

#[allow(warnings)]
use std::ops::Deref;

#[test]
fn t0() {
    fn hello(name: &str) {
        println!("Hello, {name}!");
    }

    let s = String::from("hello world");
    // String实现了Deref特质(impl ops::Deref for String { type Target = str;)
    hello(&s); // &String ---> &str

    let s_box = Box::new(String::from("Rust"));
    // 1. Box实现了Deref特质(impl ops::Deref for String { type Target = str;)
    // 2. String实现了Deref特质(impl<T: ?Sized, A: Allocator> Deref for Box<T, A> { type Target = T;)
    hello(&s_box); // &Box<String> ---> &String ---> &str
}

#[test]
fn t1() {
    struct Foo;

    impl Foo {
        fn foo(&self) {
            println!("Foo");
        }
    }

    let f = Foo;

    f.foo(); // dot(.)运算符按需对左操作数隐式引用和解引用
    /*
    &T的Deref特质实现如下:
    ```rust
    impl<T: ?Sized> Deref for &T {
        type Target = T;

        #[rustc_diagnostic_item = "noop_method_deref"]
        fn deref(&self) -> &T {
            *self
        }
    } 
    ```
     */
    (&&f).foo(); // &&f(Deref coercion) ---> &f
    (&&&f).foo(); // &&&f(Deref coercion) ---> &&f(Deref coercion) ---> &f
    (&&&&f).foo(); 
}