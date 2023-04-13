## The Static Lifetime

One special lifetime we need to discuss is 'static, which denotes that the affected reference can live for the entire duration of the program. All string literals have the 'static lifetime, which we can annotate as follows:
```rust
fn main() {
    let s: &'static str = "I have a static lifetime.";
}
```

The text of this string is stored directly in the program’s binary, which is always available. Therefore, the lifetime of all string literals is 'static.