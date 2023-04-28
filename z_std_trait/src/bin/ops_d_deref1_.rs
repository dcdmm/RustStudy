// Trait std::ops::Deref
/*
Deref coercion converts a reference to a type that implements the Deref trait into a reference to another type.
For example, deref coercion can convert &String to &str because String implements the Deref trait such that it returns &str.
Deref coercion is a convenience Rust performs on arguments to functions and methods, and works only on types that implement the Deref trait.
It happens automatically when we pass a reference to a particular type’s value as an argument to a function or method that doesn’t match the parameter type in the function or method definition.
A sequence of calls to the deref method converts the type we provided into the type the parameter needs.

Deref coercion was added to Rust so that programmers writing function and method calls don’t need to add as many explicit references and dereferences with & and *.
The deref coercion feature also lets us write more code that can work for either references or smart pointers.
 */

/*
Similar to how you use the Deref trait to override the * operator on immutable references, you can use the DerefMut trait to override the * operator on mutable references.

Rust does deref coercion when it finds types and trait implementations in three cases:
* From &T to &U when T: Deref<Target=U>
* From &mut T to &mut U when T: DerefMut<Target=U>
* From &mut T to &U when T: Deref<Target=U>

The first two cases are the same as each other except that the second implements mutability.
The first case states that if you have a &T, and T implements Deref to some type U, you can get a &U transparently.
The second case states that the same deref coercion happens for mutable references.

The third case is trickier: Rust will also coerce a mutable reference to an immutable one.
But the reverse is not possible: immutable references will never coerce to mutable references.
Because of the borrowing rules, if you have a mutable reference, that mutable reference must be the only reference to that data (otherwise, the program wouldn’t compile).
Converting one mutable reference to one immutable reference will never break the borrowing rules.
Converting an immutable reference to a mutable reference would require that the initial immutable reference is the only immutable reference to that data, but the borrowing rules don’t guarantee that.
Therefore, Rust can’t make the assumption that converting an immutable reference to a mutable reference is possible.
 */
fn hello(name: &str) {
    println!("Hello, {name}!");
}

fn main() {
    let m = Box::new(String::from("Rust"));
    hello("Rust");

    /*
    Rust can turn &Box<String> into &String by calling deref.
    The standard library provides an implementation of Deref on String that returns a string slice, and this is in the API documentation for Deref.
    Rust calls deref again to turn the &String into &str, which matches the hello function’s definition.
     */
    hello(&m);
    /*
    The (*m) dereferences the Box<String> into a String.
    Then the & and [..] take a string slice of the String that is equal to the whole string to match the signature of hello.
     */
    hello(&(*m)[..]); // 与上等价
}
