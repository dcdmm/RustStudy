// Deref


struct MyBox<T>(T);

impl<T> MyBox<T> {
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

fn hello(name: &str) {
    println!("Hello, {name}!");
}

use std::ops::Deref;

// Without the Deref trait, the compiler can only dereference & references. 
impl<T> Deref for MyBox<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn main() {
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y); // 解引用
    // no implementation for `{integer} == &{integer}`
    // assert_eq!(5, y);

    let x1 = 5;
    let y1 = Box::new(x);

    assert_eq!(5, x1);
    assert_eq!(5, *y1); // 解引用

    let x2 = 5;
    let y2 = MyBox::new(x2);

    assert_eq!(5, x2);
    assert_eq!(5, *y2); // 等价于assert_eq!(5, *(y2.deref()))

    // Deref coercion converts a reference to a type that implements the Deref trait into a reference to another type. For example, deref coercion can convert &String to &str because String implements the Deref trait such that it returns &str. Deref coercion is a convenience Rust performs on arguments to functions and methods, and works only on types that implement the Deref trait. It happens automatically when we pass a reference to a particular type’s value as an argument to a function or method that doesn’t match the parameter type in the function or method definition. A sequence of calls to the deref method converts the type we provided into the type the parameter needs.
    // &m: &MyBox<String>  --deref-->&String --deref-->&str
    let m = MyBox::new(String::from("Rust"));
    hello(&m);

    // if Rust didn’t implement deref coercion
    hello(&(*m)[..]);

    // Similar to how you use the Deref trait to override the * operator on immutable references, you can use the DerefMut trait to override the * operator on mutable references.

    //  Rust does deref coercion when it finds types and trait implementations in three cases:

    // * From &T to &U when T: Deref<Target=U>
    // * From &mut T to &mut U when T: DerefMut<Target=U>
    // * From &mut T to &U when T: Deref<Target=U>
}