Lifetime Annotation Syntax:

* a reference:&i32
* a reference with an explicit lifetime:&'a i32
* a mutable reference with an explicit lifetime:&'a mut i32

Rather than ensuring that a type has the behavior we want, lifetimes ensure that references are valid as long as we need
them to be.

```rust
// Note that we want the function to take string slices, which are references, rather than strings, because we don’t want the longest function to take ownership of its parameters.
fn longest(x: &str, y: &str) -> &str {
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

/*
程序运行结果(报错):
error[E0106]: missing lifetime specifier
 --> d_advanced\src\main.rs:1:33
  |
1 | fn longest(x: &str, y: &str) -> &str {
  |               ----     ----     ^ expected named lifetime parameter
  |
  = help: this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from `x` or `y`
help: consider introducing a named lifetime parameter
  |
1 | fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
  |           ++++     ++          ++          ++
 */
```

```rust
/*
The function signature now tells Rust that for some lifetime 'a, the function takes two parameters, both of which are string slices that live at least as long as lifetime 'a. 
The function signature also tells Rust that the string slice returned from the function will live at least as long as lifetime 'a.
In practice, it means that the lifetime of the reference returned by the longest function is the same as the smaller of the lifetimes of the values referred to by the function arguments. 

Remember, when we specify the lifetime parameters in this function signature, we’re not changing the lifetimes of any values passed in or returned. 
Rather, we’re specifying that the borrow checker should reject any values that don’t adhere to these constraints. Note that the longest function doesn’t need to know exactly how long x and y will live, only that some scope can be substituted for 'a that will satisfy this signature.

When annotating lifetimes in functions, the annotations go in the function signature, not in the function body. 
The lifetime annotations become part of the contract of the function, much like the types in the signature.

When we pass concrete references to longest, the concrete lifetime that is substituted for 'a is the part of the scope of x that overlaps with the scope of y. 
In other words, the generic lifetime 'a will get the concrete lifetime that is equal to the smaller of the lifetimes of x and y. 
Because we’ve annotated the returned reference with the same lifetime parameter 'a, the returned reference will also be valid for the length of the smaller of the lifetimes of x and y.
 */
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

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

/*
In this example, string1 is valid until the end of the outer scope, string2 is valid until the end of the inner scope, and result references something that is valid until the end of the inner scope. 
Run this code, and you’ll see that the borrow checker approves; 
it will compile and print The longest string is long string is long.
 */
fn main() {
    let string1 = String::from("long string is long");

    {
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }
}
```

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

/*
The error shows that for result to be valid for the println! statement, string2 would need to be valid until the end of the outer scope. 
Rust knows this because we annotated the lifetimes of the function parameters and return values using the same lifetime parameter 'a.
 */
fn main() {
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
    }
    println!("The longest string is {}", result);
}

/*
程序运行结果(报错):
error[E0597]: `string2` does not live long enough
  --> d_advanced\src\main.rs:14:44
   |
14 |         result = longest(string1.as_str(), string2.as_str());
   |                                            ^^^^^^^^^^^^^^^^ borrowed value does not live long enough
15 |     }
   |     - `string2` dropped here while still borrowed
16 |     println!("The longest string is {}", result);
   |                                          ------ borrow later used here
 */
```

```rust
/*
The way in which you need to specify lifetime parameters depends on what your function is doing. 
For example, if we changed the implementation of the longest function to always return the first parameter rather than the longest string slice, we wouldn’t need to specify a lifetime on the y parameter. 
The following code will compile:
 */
#[allow(dead_code)]
fn longest<'a>(x: &'a str, y: &str) -> &'a str { // We’ve specified a lifetime parameter 'a for the parameter x and the return type, but not for the parameter y, because the lifetime of y does not have any relationship with the lifetime of x or the return value.
    x
}

fn main() {
    let string1 = String::from("abcd");
    let string2 = "efghijklmnopqrstuvwxyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}
```

```rust
/*
When returning a reference from a function, the lifetime parameter for the return type needs to match the lifetime parameter for one of the parameters.
If the reference returned does not refer to one of the parameters, it must refer to a value created within this function.
However, this would be a dangling reference because the value will go out of scope at the end of the function.
 */
#[allow(dead_code)]
fn longest<'a>(_x: &str, y: &str) -> &'a str {
    let result = String::from("really long string");
    result.as_str()  // 改为:result
}

fn main() {}

/*
Here, even though we’ve specified a lifetime parameter 'a for the return type, this implementation will fail to compile because the return value lifetime is not related to the lifetime of the parameters at all. 
Here is the error message we get:

程序运行结果(报错):
error[E0515]: cannot return reference to local variable `result`
  --> c_advanced\src\bin\test_.rs:15:5
   |
 6 |     result.as_str()
   |     ^^^^^^^^^^^^^^^ returns a reference to data owned by the current function

The problem is that result goes out of scope and gets cleaned up at the end of the longest function. 
We’re also trying to return a reference to result from the function. 
There is no way we can specify lifetime parameters that would change the dangling reference, and Rust won’t let us create a dangling reference. 
In this case, the best fix would be to return an owned data type rather than a reference so the calling function is then responsible for cleaning up the value.
 */
```