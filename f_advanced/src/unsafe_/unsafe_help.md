Unsafe operations are those that can potentially violate the memory-safety guarantees of Rust's static semantics.

The following language level features cannot be used in the safe subset of Rust:
* Dereferencing a raw pointer.
* Reading or writing a mutable or external static variable.
* Accessing a field of a union, other than to assign to it.
* Calling an unsafe function (including an intrinsic or foreign function).
* Implementing an unsafe trait.
  
The unsafe keyword can occur in several different contexts: unsafe functions (unsafe fn), unsafe blocks (unsafe {}), unsafe traits (unsafe trait), and unsafe trait implementations (unsafe impl).