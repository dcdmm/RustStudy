// Unrecoverable Errors with panic!

fn main() {
    // panic!("crash and burn"); // Panics the current thread.(类似python raise关键字的作用)

    let v = vec![1, 2, 3];
    v[99];
}
