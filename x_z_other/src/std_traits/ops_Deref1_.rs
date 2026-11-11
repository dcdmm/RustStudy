// Trait std::ops::Deref/Implicit Deref Coercions with Functions and Methods

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

#[test]
fn t2() {
    struct MyBox<T>(T);

    impl<T> Deref for MyBox<T> {
        type Target = T;

        fn deref(&self) -> &T {
            println!("MyBox::deref被调用");
            &self.0
        }
    }

    struct Dog {
        name: String,
    }

    impl Dog {
        fn bark(&self) {
            println!("{} 汪汪", self.name);
        }
    }

    let d = MyBox(Dog {name: String::from("旺财")});
    d.bark(); // 等价于: Dog::bark(Deref::deref(&d))
    /*
    bark的签名为fn bark(&self),要求接收者类型为&Dog
    1. *d: Dog // MyBox实现了Deref<Target = Dog>,解引用一层
    2. &*d: &Dog // 匹配
    3. Dog::bark(&*d)
    4. Dog::bark(&*Deref::deref(&d)) // *d = *Deref::deref(&d)
    5. Dog::bark(Deref::deref(&d))
     */
}