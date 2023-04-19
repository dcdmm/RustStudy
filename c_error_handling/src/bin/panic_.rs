// Unrecoverable Errors with panic!

fn main() {
    // panic:thread 'main' panicked at 'crash and burn'
    // panic!("crash and burn") // Panics the current thread.(类似python raise关键字的作用)

    let v = vec![1, 2, 3];
    // panic:thread 'main' panicked at 'index out of bounds: the len is 3 but the index is 99'
    v[99];
}
