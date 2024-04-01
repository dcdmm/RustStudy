// Trait std::ops::Deref

/*
Used for immutable dereferencing operations, like *v.

In addition to being used for explicit dereferencing operations with the (unary) * operator in immutable contexts, Deref is also used implicitly by the compiler in many circumstances. This mechanism is called ‘Deref coercion’. In mutable contexts, DerefMut is used.

Warning: Deref coercion is a powerful language feature which has far-reaching implications for every type that implements Deref. The compiler will silently insert calls to Deref::deref. For this reason, one should be careful about implementing Deref and only do so when deref coercion is desirable. See below for advice on when this is typically desirable or undesirable.

Types that implement Deref or DerefMut are often called “smart pointers” and the mechanism of deref coercion has been specifically designed to facilitate the pointer-like behaviour that name suggests.


 */

#[allow(warnings)]
use std::ops::Deref;

#[test]
fn t0() {
    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y); // *:解引用运算符

    // error[E0277]: can't compare `{integer}` with `&{integer}`
    // assert_eq!(5, y);
}

#[test]
fn t1() {
    let x = 5;
    let y = Box::new(x); // Box实现了Iterator特质

    assert_eq!(5, x);
    assert_eq!(5, *y); 
}

#[allow(warnings)]
#[test]
fn t2_0() {
    struct MyBox<T>(T); // tuple structs

    impl<T> MyBox<T> {
        fn new(x: T) -> MyBox<T> {
            MyBox(x)
        }
    }
    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    
    // error[E0614]: type `ops_Deref0_::t2_0::MyBox<{integer}>` cannot be dereferenced
    // assert_eq!(5, *y);
}

#[test]
fn t2_1() {
    struct MyBox<T>(T); // tuple structs

    impl<T> MyBox<T> {
        fn new(x: T) -> MyBox<T> {
            MyBox(x)
        }
    }

    impl<T> Deref for MyBox<T> {
        // The type Target = T; syntax defines an associated type for the Deref trait to use.
        type Target = T;

        // Without the Deref trait, the compiler can only dereference & references. The deref method gives the compiler the ability to take a value of any type that implements Deref and call the deref method to get a & reference that it knows how to dereference.
        fn deref(&self) -> &Self::Target {
            /*
            We fill in the body of the deref method with &self.0 so deref returns a reference to the value we want to access with the * operator;
            
            The reason the deref method returns a reference to a value, and that the plain dereference outside the parentheses in *(y.deref()) is still necessary, is to do with the ownership system. If the deref method returned the value directly instead of a reference to the value, the value would be moved out of self. We don’t want to take ownership of the inner value inside MyBox<T> in this case or in most cases where we use the dereference operator.
             */
            &self.0
        }
    }

    let x = 5;
    let y = MyBox::new(x);

    // Rust substitutes the * operator with a call to the deref method and then a plain dereference so we don’t have to think about whether or not we need to call the deref method. This Rust feature lets us write code that functions identically whether we have a regular reference or a type that implements Deref.
    assert_eq!(5, *y); // 实际执行为*(y.deref())
}
