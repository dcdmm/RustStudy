## Lifetime Elision

the compiler uses three rules to figure out the lifetimes of the references when there aren’t explicit annotations. The first rule applies to input lifetimes, and the second and third rules apply to output lifetimes. If the compiler gets to the end of the three rules and there are still references for which it can’t figure out lifetimes, the compiler will stop with an error. These rules apply to fn definitions as well as impl blocks.

1. The first rule is that the compiler assigns a lifetime parameter to each parameter that’s a reference. In other words, a function with one parameter gets one lifetime parameter: fn foo<'a>(x: &'a i32); a function with two parameters gets two separate lifetime parameters: fn foo<'a, 'b>(x: &'a i32, y: &'b i32); and so on.

2. The second rule is that, if there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters: fn foo<'a>(x: &'a i32) -> &'a i32.

3. The third rule is that, if there are multiple input lifetime parameters, but one of them is &self or &mut self because this is a method, the lifetime of self is assigned to all output lifetime parameters. This third rule makes methods much nicer to read and write because fewer symbols are necessary.

```rust
// example 1:

fn first_word(s: &str) -> &str {  // 实际项目中的手写代码

// ======>applies the first rule
fn first_word(s: &'a str) -> &str {

// The second rule applies because there is exactly one input lifetime.
// ======>applies the second rule 
fn first_word<'a>(s: &'a str) -> &'a str { 
```

```rust
// example2:

fn longest(x: &str, y: &str) -> &str { // 实际项目中的手写代码

// ======>applies the first rule
fn longest(x: &'a str, y: &'b str) -> &str { 

/*
the second rule doesn’t apply because there is more than one input lifetime. 
The third rule doesn’t apply either, because longest is a function rather than a method

After working through all three rules, we still haven’t figured out what the return type’s lifetime is. 
This is why we got an error trying to compile the code in exaple2: the compiler worked through the lifetime elision rules but still couldn’t figure out all the lifetimes of the references in the signature.
 */
```