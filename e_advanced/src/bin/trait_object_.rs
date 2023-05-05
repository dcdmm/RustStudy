// Using Trait Objects That Allow for Values of Different Types
/*
The code that results from monomorphization is doing static dispatch, which is when the compiler knows what method you’re calling at compile time.
This is opposed to dynamic dispatch, which is when the compiler can’t tell at compile time which method you’re calling.
In dynamic dispatch cases, the compiler emits code that at runtime will figure out which method to call.

When we use trait objects, Rust must use dynamic dispatch.
The compiler doesn’t know all the types that might be used with the code that’s using trait objects, so it doesn’t know which method implemented on which type to call.
Instead, at runtime, Rust uses the pointers inside the trait object to know which method to call.
This lookup incurs a runtime cost that doesn’t occur with static dispatch.
Dynamic dispatch also prevents the compiler from choosing to inline a method’s code, which in turn prevents some optimizations.
 */

pub trait Draw {
    fn draw(&self);
}

/*
dyn is a prefix of a trait object’s type.

The dyn keyword is used to highlight that calls to methods on the associated Trait are dynamically dispatched.
To use the trait this way, it must be ‘object safe’(trait所有方法必须满足:1. 方法的返回值类型不能是Self; 2.方法没有任何泛型参数).
 */
pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

/*
//  A generic type parameter can only be substituted with one concrete type at a time, whereas trait objects allow for multiple concrete types to fill in for the trait object at runtime.

pub struct Screen<T: Draw> {
    pub components: Vec<T>,
}

impl<T> Screen<T>
where
    T: Draw,
{
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}
 */

pub struct Button {
    #[allow(dead_code)]
    pub width: u32,
    #[allow(dead_code)]
    pub height: u32,
    #[allow(dead_code)]
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!("Button draw!")
    }
}

struct SelectBox {
    #[allow(dead_code)]
    width: u32,
    #[allow(dead_code)]
    height: u32,
    #[allow(dead_code)]
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!("SelectBox draw!")
    }
}

/*
When we wrote the library, we didn’t know that someone might add the SelectBox type, but our Screen implementation was able to operate on the new type and draw it because SelectBox implements the Draw trait, which means it implements the draw method.

This concept—of being concerned only with the messages a value responds to rather than the value’s concrete type—is similar to the concept of duck typing in dynamically typed languages: if it walks like a duck and quacks like a duck, then it must be a duck!

The advantage of using trait objects and Rust’s type system to write code similar to code using duck typing is that we never have to check whether a value implements a particular method at runtime or worry about getting errors if a value doesn’t implement a method but we call it anyway.
Rust won’t compile our code if the values don’t implement the traits that the trait objects need.
 */
fn main() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ],
    };
    screen.run();
}