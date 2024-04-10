// Trait std::ops::Deref

/*
Used for immutable dereferencing operations, like *v.

In addition to being used for explicit dereferencing operations with the (unary) * operator in immutable contexts, Deref is also used implicitly by the compiler in many circumstances. This mechanism is called ‘Deref coercion’. In mutable contexts, DerefMut is used.

# Deref coercion
Rust does deref coercion when it finds types and trait implementations in three cases:
    * From &T to &U when T: Deref<Target=U>
    * From &mut T to &mut U when T: DerefMut<Target=U>
    * From &mut T to &U when T: Deref<Target=U>

    
# The dereference operator
The * (dereference) operator is also a unary prefix operator. When applied to a pointer(References (& and &mut)、Raw pointers (*const and *mut)、Smart Pointers) it denotes the pointed-to location.

On non-pointer types *x is equivalent to *std::ops::Deref::deref(&x) in an immutable place expression context and *std::ops::DerefMut::deref_mut(&mut x) in a mutable place expression context.
*/

// # Implementors
/*
impl<T: ?Sized> Deref for &T {
    type Target = T;

    #[rustc_diagnostic_item = "noop_method_deref"]
    fn deref(&self) -> &T {
        *self
    }
}

impl<T: ?Sized> Deref for &mut T {
    type Target = T;

    fn deref(&self) -> &T {
        *self
    }
}

impl<T, A: Allocator> ops::Deref for Vec<T, A> {
    type Target = [T];

impl ops::Deref for String {
    type Target = str;

impl<T: ?Sized, A: Allocator> Deref for Box<T, A> {
    type Target = T;

impl<T: ?Sized, A: Allocator> Deref for Rc<T, A> {
    type Target = T;
*/

#[allow(warnings)]
use std::ops::Deref;

#[test]
fn t0() {
    use std::rc::Rc;

    let x = 5;
    let x_ref = &x;
    let x_mut_ptr = &x as *const i32;
    let x_box = Box::new(x);
    let x_rc = Rc::new(x);

    assert_eq!(5, x);
    // error[E0277]: can't compare `{integer}` with `&{integer}`  
    // assert_eq!(5, x_ref);
    assert_eq!(5, *x_ref);
    unsafe {
        assert_eq!(5, *x_mut_ptr);
    }
    assert_eq!(5, *x_box);
    assert_eq!(5, *x_rc);
}

#[allow(warnings)]
#[test]
fn t1_0() {
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
fn t1_1() {
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
    let y = MyBox::new(x); // y不是Pointer types

    // Rust substitutes the * operator with a call to the deref method and then a plain dereference so we don’t have to think about whether or not we need to call the deref method. This Rust feature lets us write code that functions identically whether we have a regular reference or a type that implements Deref.
    assert_eq!(5, *y); // 实际执行为: *Deref::deref(&y) 或 *(y.deref())
}
