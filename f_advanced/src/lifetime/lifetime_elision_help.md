Lifetimes on function or method parameters are called input lifetimes, and lifetimes on return values are called output lifetimes.

the compiler uses three rules to figure out the lifetimes of the references when there aren’t explicit annotations. The first rule applies to input lifetimes, and the second and third rules apply to output lifetimes. If the compiler gets to the end of the three rules and there are still references for which it can’t figure out lifetimes, the compiler will stop with an error. These rules apply to fn definitions as well as impl blocks.

1. The first rule is that the compiler assigns a lifetime parameter to each parameter that’s a reference. In other words, a function with one parameter gets one lifetime parameter: fn foo<'a>(x: &'a i32); a function with two parameters gets two separate lifetime parameters: fn foo<'a, 'b>(x: &'a i32, y: &'b i32); and so on.

2. The second rule is that, if there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters: fn foo<'a>(x: &'a i32) -> &'a i32.

3. The third rule is that, if there are multiple input lifetime parameters, but one of them is &self or &mut self because this is a method, the lifetime of self is assigned to all output lifetime parameters. This third rule makes methods much nicer to read and write because fewer symbols are necessary.

* example 1:
```rust
fn first_word(s: &str) -> &str {  // 没有生命周期标注的代码

fn first_word(s: &'a str) -> &str { // 应用第一条规则,为每个参数标注一个生命周期

fn first_word<'a>(s: &'a str) -> &'a str { // 应用第二条规则,因为只有一个输入生命周期
```

* example 2:
```rust
fn longest(x: &str, y: &str) -> &str { // 没有生命周期标注的代码

fn longest(x: &'a str, y: &'b str) -> &str { // 应用第一条规则,为每个参数标注一个生命周期 

// 不能应用第二条规则,因为输入生命周期有两个
// 不能应用第三条规则,因为longest是一个函数而不是方法
```

* example 3:
```rust
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part(&self, announcement: &str) -> &str { // 没有生命周期标注的代码
        println!("Attention please: {}", announcement);
        self.part
    }
}

impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part<'b>(&'a self, announcement: &'b str) -> &str { // 应用第一条规则,为每个参数标注一个生命周期 
        println!("Attention please: {}", announcement);
        self.part
    }
}


impl<'a> ImportantExcerpt<'a> {
    fn announce_and_return_part<'b>(&'a self, announcement: &'b str) -> &'a str { // 应用第三条规则
        println!("Attention please: {}", announcement);
        self.part
    }
}
```