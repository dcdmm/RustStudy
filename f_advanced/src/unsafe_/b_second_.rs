// Reading or writing a mutable or external static variable

/*
The names of static variables are in SCREAMING_SNAKE_CASE by convention. Static variables can only store references with the 'static lifetime, which means the Rust compiler can figure out the lifetime and we aren’t required to annotate it explicitly. Accessing an immutable static variable is safe.

A subtle difference between constants and immutable static variables is that values in a static variable have a fixed address in memory. Using the value will always access the same data. Constants, on the other hand, are allowed to duplicate their data whenever they’re used. Another difference is that static variables can be mutable. Accessing and modifying mutable static variables is unsafe.
*/

static HELLO_WORLD: &str = "Hello, world!"; // 不可变静态变量

static mut COUNTER: i32 = 0; // 可变静态变量

#[test]
fn t0() {
    println!("{}", HELLO_WORLD);

    // error[E0133]: use of mutable static is unsafe and requires unsafe function or block
    // let x = COUNTER;

    let y = unsafe { COUNTER };
}

static mut RESULT: u32 = 0;

fn add_to_count(inc: u32) {
    unsafe {
        RESULT += inc;
    }
}

#[test]
fn t1() {
    add_to_count(3);
    let result = unsafe { RESULT };
    println!("{}", result) // print->3
}
