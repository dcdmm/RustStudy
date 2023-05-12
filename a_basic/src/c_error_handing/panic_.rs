// Unrecoverable Errors with panic!

#[test]
fn t0() {
    // panic:thread 'c_error_handing::panic_::t0' panicked at 'crash and burn'
    // panic!("crash and burn"); // Panics the current thread.(类似python raise关键字的作用)

    let v = vec![1, 2, 3];
    // panic:thread 'c_error_handing::panic_::t0' panicked at 'index out of bounds: the len is 3 but the index is 99'
    v[99];
}
