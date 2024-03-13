### The Borrow Checker

```rust
fn main() {
    let r;                                              // ---------+-- 'a
                                                        //          |
    {                                                   //          |
        let x = 5;                                      // -+-- 'b  | // `x`: binding `x` declared here
        // error[E0597]: `x` does not live long enough  //
        r = &x;                                         //  |       | // `&x`: borrowed value does not live long enough
    }                                                   // -+       | // `}`: `x` dropped here while still borrowed
                                                        //          | 
    println!("r: {}", r);                               //          | // `r`: borrow later used here
}
```

At compile time, Rust compares the size of the two lifetimes and sees that r has a lifetime of 'a but that it refers to memory with a lifetime of 'b. The program is rejected because 'b is shorter than 'a: the subject of the reference doesn’t live as long as the reference.

***

```rust
fn main() {
    let x = 5;            // ----------+-- 'b
                          //           |
    let r = &x;           // --+-- 'a  |
                          //   |       |
    println!("r: {}", r); //   |       |
                          // --+       |
}                         // ----------+
```

Here, x has the lifetime 'b, which in this case is larger than 'a. This means r can reference x because Rust knows that the reference in r will always be valid while x is valid.

### Lifetime Annotation Syntax

Lifetime annotations don’t change how long any of the references live. Rather, they describe the relationships of the lifetimes of multiple references to each other without affecting the lifetimes. Just as functions can accept any type when the signature specifies a generic type parameter, functions can accept references with any lifetime by specifying a generic lifetime parameter.

Here are some examples: a reference to an i32 without a lifetime parameter, a reference to an i32 that has a lifetime parameter named 'a, and a mutable reference to an i32 that also has the lifetime 'a.
```rust
&i32 // a reference
&'a i32 // a reference with an explicit lifetime:
&'a mut i32 // a mutable reference with an explicit lifetime:
```

### Lifetime Annotations in Function Signatures

```rust
// error[E0106]: missing lifetime specifier
fn longest(x: &str, y: &str) -> &str { // 无法判断返回值是x(引用;具体生命周期未知)还是y(引用;具体生命周期未知),故不能确定返回值(引用)是否始终有效
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}
```

***

To use lifetime annotations in function signatures, we need to declare the generic lifetime parameters inside angle brackets between the function name and the parameter list, just as we did with generic type parameters.

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}
```

The function signature now tells Rust that for some lifetime 'a, the function takes two parameters, both of which are string slices that live at least as long as lifetime 'a. The function signature also tells Rust that the string slice returned from the function will live at least as long as lifetime 'a. In practice, it means that the lifetime of the reference returned by the longest function is the same as the smaller of the lifetimes of the values referred to by the function arguments. 

Remember, when we specify the lifetime parameters in this function signature, we’re not changing the lifetimes of any values passed in or returned. Rather, we’re specifying that the borrow checker should reject any values that don’t adhere to these constraints. Note that the longest function doesn’t need to know exactly how long x and y will live, only that some scope can be substituted for 'a that will satisfy this signature.

When annotating lifetimes in functions, the annotations go in the function signature, not in the function body. The lifetime annotations become part of the contract of the function, much like the types in the signature.

When we pass concrete references to longest, the concrete lifetime that is substituted for 'a is the part of the scope of x that overlaps with the scope of y. In other words, the generic lifetime 'a will get the concrete lifetime that is equal to the smaller of the lifetimes of x and y. Because we’ve annotated the returned reference with the same lifetime parameter 'a, the returned reference will also be valid for the length of the smaller of the lifetimes of x and y.

***

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {  // 类型函数定义中泛型的定义
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("long string is long");

    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("{}", result);
    }
}
```

In this example, string1 is valid until the end of the outer scope, string2 is valid until the end of the inner scope, and result references something that is valid until the end of the inner scope. Run this code, and you’ll see that the borrow checker approves; it will compile and print long string is long.

***

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("long string is long");
    let result;
    {   
        // error[E0597]: `string2` does not live long enough
        let string2 = String::from("xyz"); // `string2`: binding `string2` declared here
        result = longest(string1.as_str(), string2.as_str()); // `string2`: borrowed value does not live long enough
    } // `}`: `string2` dropped here while still borrowed
    println!("The longest string is {}", result); // `result`: borrow later used here
}
```

The error shows that for result to be valid for the println! statement, string2 would need to be valid until the end of the outer scope. Rust knows this because we annotated the lifetimes of the function parameters and return values using the same lifetime parameter 'a.

### Thinking in Terms of Lifetimes

The way in which you need to specify lifetime parameters depends on what your function is doing. For example, if we changed the implementation of the longest function to always return the first parameter rather than the longest string slice, we wouldn’t need to specify a lifetime on the y parameter. The following code will compile:

```rust
fn longest<'a>(x: &'a str, y: &str) -> &'a str {  
    x
}

fn main() {
    let string1 = String::from("abcd");
    let string2 = "efghijklmnopqrstuvwxyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}
```

We’ve specified a lifetime parameter 'a for the parameter x and the return type, but not for the parameter y, because the lifetime of y does not have any relationship with the lifetime of x or the return value.

***

When returning a reference from a function, the lifetime parameter for the return type needs to match the lifetime parameter for one of the parameters. If the reference returned does not refer to one of the parameters, it must refer to a value created within this function.

```rust
fn longest<'a>(x: &str, y: &str) -> &'a str { // 
    let result = String::from("really long string");
    // error[E0515]: cannot return value referencing local variable `result`
    result.as_str()
}

fn main() {
    let string1 = String::from("abcd");
    let string2 = "efghijklmnopqrstuvwxyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}
```

Here, even though we’ve specified a lifetime parameter 'a for the return type, this implementation will fail to compile because the return value lifetime is not related to the lifetime of the parameters at all.

The problem is that result goes out of scope and gets cleaned up at the end of the longest function. We’re also trying to return a reference to result from the function. There is no way we can specify lifetime parameters that would change the dangling reference, and Rust won’t let us create a dangling reference. In this case, the best fix would be to return an owned data type rather than a reference so the calling function is then responsible for cleaning up the value.

### In Struct Definitions

```rust
struct ImportantExcerpt<'a> { // 类型结构体定义中泛型的定义
    part: &'a str,
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
}
```

This struct has the single field part that holds a string slice, which is a reference. As with generic data types, we declare the name of the generic lifetime parameter inside angle brackets after the name of the struct so we can use the lifetime parameter in the body of the struct definition. This annotation means an instance of ImportantExcerpt can’t outlive the reference it holds in its part field.

The main function here creates an instance of the ImportantExcerpt struct that holds a reference to the first sentence of the String owned by the variable novel. The data in novel exists before the ImportantExcerpt instance is created. In addition, novel doesn’t go out of scope until after the ImportantExcerpt goes out of scope, so the reference in the ImportantExcerpt instance is valid.

***

```rust
#[derive(Debug)]
struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    let i;
    {
        let novel = String::from("Call me Ishmael. Some years ago..."); // `novel`: binding `novel` declared here
        // error[E0597]: `novel` does not live long enough
        let first_sentence = novel.split('.').next().expect("Could not find a '.'"); // `novel`: borrowed value does not live long enough
        i = ImportantExcerpt {
            part: first_sentence,
        };
    } // `}`: `novel` dropped here while still borrowed
    println!("{:?}", i); // `i`: borrow later used here
}
```

### Lifetime Annotations in Method Definitions

```rust
struct ImportantExcerpt<'a> {
    part: &'a str,
}

#[allow(dead_code)]
impl<'a> ImportantExcerpt<'a> { // 类似方法定义中泛型的定义
    fn level(&self) -> i32 {
        3
    }
}
```