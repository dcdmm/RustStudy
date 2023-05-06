// unsafe

/*
To ensure soundness, Safe Rust is restricted enough that it can be automatically checked. Sometimes, however, it is necessary to write code that is correct for reasons which are too clever for the compiler to understand. In those cases, you need to use Unsafe Rust.

Here are the abilities Unsafe Rust has in addition to Safe Rust:
* Dereference raw pointers
* Implement unsafe traits
* Call unsafe functions
* Mutate statics (including external ones)
* Access fields of unions
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
fn main() {
    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;
}