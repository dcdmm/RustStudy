Unsafe operations are those that can potentially violate the memory-safety guarantees of Rust's static semantics.

The following language level features cannot be used in the safe subset of Rust:
1. Dereferencing a raw pointer.
2. Reading or writing a mutable or external static variable.
3. Accessing a field of a union, other than to assign to it.
4. Calling an unsafe function (including an intrinsic or foreign function).
5. Implementing an unsafe trait.
  
The unsafe keyword can occur in several different contexts: unsafe functions (unsafe fn), unsafe blocks (unsafe {}), unsafe traits (unsafe trait), and unsafe trait implementations (unsafe impl).