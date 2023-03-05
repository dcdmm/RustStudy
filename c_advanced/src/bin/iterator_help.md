## iterator

All iterators implement a trait named Iterator that is defined in the standard library. The definition of the trait looks like this:
```rust
pub trait Iterator {
    type Item;

    // The Iterator trait only requires implementors to define one method: the next method, which returns one item of the iterator at a time wrapped in Some and, when iteration is over, returns None.
    fn next(&mut self) -> Option<Self::Item>;

    // methods with default implementations elided
}
```