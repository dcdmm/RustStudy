// unsafe & raw pointers & static variables

/*
To ensure soundness, Safe Rust is restricted enough that it can be automatically checked.
Sometimes, however, it is necessary to write code that is correct for reasons which are too clever for the compiler to understand.
In those cases, you need to use Unsafe Rust.

Here are the abilities Unsafe Rust has in addition to Safe Rust:
1. Dereference raw pointers
2. Implement unsafe traits
3. Call an unsafe function or method
4. Mutate statics (including external ones)
5. Access fields of unions
 */


/*
Unsafe Rust has two new types called raw pointers that are similar to references.
As with references, raw pointers can be immutable or mutable and are written as *const T and *mut T, respectively.
The asterisk isn’t the dereference operator; it’s part of the type name. In the context of raw pointers, immutable means that the pointer can’t be directly assigned to after being dereferenced.

Different from references and smart pointers, raw pointers:
* Are allowed to ignore the borrowing rules by having both immutable and mutable pointers or multiple mutable pointers to the same location
* Aren’t guaranteed to point to valid memory
* Are allowed to be null
* Don’t implement any automatic cleanup
 */


/*
In Rust, global variables are called static variables.

The names of static variables are in SCREAMING_SNAKE_CASE by convention.
Static variables can only store references with the 'static lifetime, which means the Rust compiler can figure out the lifetime and we aren’t required to annotate it explicitly.
Accessing an immutable static variable is safe.

A subtle difference between constants and immutable static variables is that values in a static variable have a fixed address in memory.
Using the value will always access the same data.
Constants, on the other hand, are allowed to duplicate their data whenever they’re used.
Another difference is that static variables can be mutable. Accessing and modifying mutable static variables is unsafe.

With mutable data that is globally accessible, it’s difficult to ensure there are no data races, which is why Rust considers mutable static variables to be unsafe.

All access to a static is safe, but there are a number of restrictions on statics:
* The type must have the Sync trait bound to allow thread-safe access.
* Constants cannot refer to statics.
 */

fn main() {}

// Dereferencing a Raw Pointer
#[test]
fn one_() {
    let mut num = 5;

    // Creating raw pointers from references
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    // Dereferencing raw pointers within an unsafe block
    unsafe {
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
    }

    let address = 0x012345usize;
    let _fr = address as *const i32;
}

// Implementing an Unsafe Trait
/*
We can use unsafe to implement an unsafe trait.
A trait is unsafe when at least one of its methods has some invariant that the compiler can’t verify.
We declare that a trait is unsafe by adding the unsafe keyword before trait and marking the implementation of the trait as unsafe too, as shown in Listing 1.
 */
#[test]
fn two_() {
    // Listing 1: Defining and implementing an unsafe trait

    unsafe trait Foo {
        // methods go here
    }

    unsafe impl Foo for i32 {
        // method implementations go here
    }
}

// Calling an Unsafe Function or Method
#[test]
fn three_() {
    unsafe fn dangerous() {}

    // We must call the dangerous function within a separate unsafe block.
    unsafe {
        dangerous();
    }
}

#[test]
fn two_1() {
    use std::slice;

    // Using unsafe code in the implementation of the split_at_mut function
    fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
        let len = values.len();
        let ptr = values.as_mut_ptr();

        assert!(mid <= len);

        unsafe {
            (
                slice::from_raw_parts_mut(ptr, mid),
                slice::from_raw_parts_mut(ptr.add(mid), len - mid),
            )
        }
    }

    let mut v = vec![1, 2, 3, 4, 5, 6];

    let r = &mut v[..];

    let (a, b) = split_at_mut(r, 3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);
}

/*
Accessing non-mut(https://doc.rust-lang.org/std/keyword.mut.html) static items is considered safe, but some restrictions apply.
Most notably, the type of a static value needs to implement the Sync(https://doc.rust-lang.org/std/marker/trait.Sync.html) trait, ruling out interior mutability containers like RefCell(https://doc.rust-lang.org/std/cell/struct.RefCell.html).
See the Reference(https://doc.rust-lang.org/reference/items/static-items.html) for more information.
 */
#[test]
fn simple_statics_example() {
    static FOO: [i32; 5] = [1, 2, 3, 4, 5];

    let r1 = &FOO as *const _;
    let r2 = &FOO as *const _;
    // With a strictly read-only static, references will have the same address
    assert_eq!(r1, r2);
    // A static item can be used just like a variable in many cases
    println!("{FOO:?}");
}

// Accessing or Modifying a Mutable Static Variable
#[test]
fn four_() {
    static mut COUNTER: u32 = 0;

    fn add_to_count(inc: u32) {
        unsafe {
            COUNTER += inc;
        }
    }

    add_to_count(3);

    unsafe {
        println!("COUNTER: {}", COUNTER);
    }
}

// Accessing Fields of a Union
#[test]
fn five_() {
    /*
    A union is similar to a struct, but only one declared field is used in a particular instance at one time.
    Unions are primarily used to interface with unions in C code.
    Accessing union fields is unsafe because Rust can’t guarantee the type of the data currently being stored in the union instance.
     */

    // 参考:https://doc.rust-lang.org/reference/items/unions.html
}