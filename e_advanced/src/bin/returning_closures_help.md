Closures are represented by traits, which means you can’t return closures directly. In most cases where you might want
to return a trait, you can instead use the concrete type that implements the trait as the return value of the function.
However, you can’t do that with closures because they don’t have a concrete type that is returnable; you’re not allowed
to use the function pointer fn as a return type, for example.

The following code tries to return a closure directly, but it won’t compile:

```rust
fn returns_closure() -> dyn Fn(i32) -> i32 {
    |x| x + 1
}
fn main() {}
/*
程序运行结果(报错):
error[E0746]: return type cannot have an unboxed trait object
 --> e_advanced\src\main.rs:4:25
  |
4 | fn returns_closure() -> dyn Fn(i32) -> i32 {
  |                         ^^^^^^^^^^^^^^^^^^ doesn't have a size known at compile-time
  |
  = note: for information on `impl Trait`, see <https://doc.rust-lang.org/book/ch10-02-traits.html#returning-types-that-implement-traits>
help: use `impl Fn(i32) -> i32` as the return type, as all return paths are of type `[closure@e_advanced\src\main.rs:5:5: 5:8]`, which implements `Fn(i32) -> i32`
  |
4 | fn returns_closure() -> impl Fn(i32) -> i32 {
  |                         ~~~~~~~~~~~~~~~~~~~
 */
```

The error references the Sized trait again! Rust doesn’t know how much space it will need to store the closure. We saw a solution to this problem earlier. We can use a trait object:

```rust
fn returns_closure() -> Box<dyn Fn(i32) -> i32> {
    Box::new(|x| x + 1)
}

fn main() {}
```