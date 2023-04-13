```rust
/*
At compile time, Rust compares the size of the two lifetimes and sees that r has a lifetime of 'a but that it refers to memory with a lifetime of 'b. 
The program is rejected because 'b is shorter than 'a: the subject of the reference doesn’t live as long as the reference.
 */
fn main() {
    let r;                // ---------+-- 'a
                          //          |
    {                     //          |
        let x = 5;        // -+-- 'b  |
        r = &x;           //  |       |
    }                     // -+       |
                          //          |
    println!("r: {}", r); //          |
}

/*
程序运行结果(报错):
error[E0597]: `x` does not live long enough
 --> d_advanced\src\main.rs:6:13
  |
6 |         r = &x;            //  |       |
  |             ^^ borrowed value does not live long enough
7 |     }                      // -+       |
  |     - `x` dropped here while still borrowed
8 |                            //          |
9 |     println!("r: {}", r);  //          |
  |                       - borrow later used here
 */
```

```rust
/*
Here, x has the lifetime 'b, which in this case is larger than 'a. 
This means r can reference x because Rust knows that the reference in r will always be valid while x is valid.
 */
fn main() {
    let x = 5;            // ----------+-- 'b
                          //           |
    let r = &x;           // --+-- 'a  |
                          //   |       |
    println!("r: {}", r); //   |       |
                          // --+       |
}                         // ----------+
```