// Calling an unsafe function (including an intrinsic or foreign function).

#[test]
fn t0() {
    unsafe fn deref_unchecked(ptr: *const i32) -> i32 {
        unsafe { *ptr }
    }

    let a = 3;
    let b = &a as *const _;

    unsafe {
        assert_eq!(*b, deref_unchecked(b));
    };
}

#[test]
fn t1() {
    unsafe fn modify_array(ptr: *mut i32, len: usize) {
        if ptr.is_null() {
            return;
        }
        for i in 0..len {
            let elem = ptr.offset(i as isize);
            let val = *elem;
            *elem = val + 10;
        }
    }

    let mut numbers = [1, 2, 3, 4, 5];
    unsafe {
        modify_array(numbers.as_mut_ptr(), numbers.len());
    }

    println!("Modified array: {:?}", numbers);
}
