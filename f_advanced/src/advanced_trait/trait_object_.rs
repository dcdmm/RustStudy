// Using Trait Objects That Allow for Values of Different Types

/*
The code that results from monomorphization is doing static dispatch, which is when the compiler knows what method you’re calling at compile time. This is opposed to dynamic dispatch, which is when the compiler can’t tell at compile time which method you’re calling. In dynamic dispatch cases, the compiler emits code that at runtime will figure out which method to call.

When we use trait objects, Rust must use dynamic dispatch. The compiler doesn’t know all the types that might be used with the code that’s using trait objects, so it doesn’t know which method implemented on which type to call. Instead, at runtime, Rust uses the pointers inside the trait object to know which method to call. This lookup incurs a runtime cost that doesn’t occur with static dispatch. Dynamic dispatch also prevents the compiler from choosing to inline a method’s code, which in turn prevents some optimizations.
 */

pub trait Draw {
    fn draw(&self);
}

#[allow(warnings)]
pub struct Screen {
    /*
    dyn is a prefix of a trait object’s type.

    The dyn keyword is used to highlight that calls to methods on the associated Trait are dynamically dispatched. To use the trait this way, it must be dyn compatible.
     */
    pub components: Vec<Box<dyn Draw>>,
}

#[allow(warnings)]
impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        println!("Button draw!")
    }
}

#[allow(warnings)]
struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!("SelectBox draw!")
    }
}

#[test]
fn t0() {
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


#[test]
fn t1() {
    // 错误原因: Be a method that does not use Self except in the type of the receiver
    // trait CopyDrawBad {
    //     fn duplicate(&self) -> Self;
    // }
    // // error[E0038]: the trait `CopyDrawBad` is not dyn compatible
    // fn show_copy_bad(component: &dyn CopyDrawBad) {}

    trait CopyDrawFixed {
        fn draw(&self);
        fn duplicate(&self) -> Self
        where
            Self: Sized;
    }
    fn show_copy_fiexed(component: &dyn CopyDrawFixed) {
        // 错误原因: Explicitly non-dispatchable functions require: Have a where Self: Sized bound (receiver type of Self (i.e. self) implies this).
        // 
        // error: the `duplicate` method cannot be invoked on a trait object
        // let copied = component.duplicate();
    }

    trait CopyDrawBoxFixed {
        fn draw(&self);
        fn duplicate(&self) -> Box<dyn CopyDrawBoxFixed>;
    }
    fn show_copy_boxed(component: &dyn CopyDrawBoxFixed) {
        let copied = component.duplicate();
    }
}

#[test]
fn t2() {
    // 错误原因: Not have any type parameters (although lifetime parameters are allowed)
    // 
    // trait TextDrawBad {
    //     fn draw<T>(&self, value: T);
    // }
    // // error[E0038]: the trait `TextDrawBad` is not dyn compatible
    // fn show_text_bad(component: &dyn TextDrawBad) {}

    trait TextDrawFixed {
        fn draw(&self, value: &str);
    }
    fn show_text_fixed(component: &dyn TextDrawFixed) {}
}

#[test]
fn t3() {
    // 错误原因: Have a receiver with one of the following types:
    // * &Self (i.e. &self)
    // * &mut Self (i.e &mut self)
    // * Box<Self>
    // * Rc<Self>
    // * Arc<Self>
    // * Pin<P> where P is one of the types above
    //
    // trait CategoryDrawBad {
    //     fn category() -> u8;
    // }
    // // error[E0038]: the trait `CategoryDrawBad` is not dyn compatible
    // fn show_category_bad(component: &dyn CategoryDrawBad) {}

    trait CategoryDrawFixed {
        fn category(&self) -> u8;
    }
    fn show_category_fixed(component: &dyn CategoryDrawFixed) {}
}
