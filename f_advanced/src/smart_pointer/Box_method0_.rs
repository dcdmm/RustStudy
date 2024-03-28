// std::boxed::Box

/*
常见方法:
new
leak
 */

#[test]
fn new_test() {
    /*
    pub fn new(x: T) -> Box<T>
        Allocates memory on the heap and then places x into it.

        This doesn’t actually allocate if T is zero-sized.
     */
    let five = Box::new(5);
    println!("{}", five);
}

#[test]
fn leak_test() {
    let x = Box::new(41);
    /*
    pub fn leak<'a>(b: Self) -> &'a mut T
    where
        A: 'a,

    Consumes and leaks the Box, returning a mutable reference, &'a mut T. Note that the type T must outlive the chosen lifetime 'a. If the type has only static references, or none at all, then this may be chosen to be 'static.
     */
    let static_ref = Box::leak(x); // `x`: value moved here
    *static_ref += 1;
    println!("{}", static_ref);
    // error[E0382]: borrow of moved value: `x`
    // println!("{}", x);

    let y = vec![1, 2, 3].into_boxed_slice();
    let static_ref_y = Box::leak(y);
    println!("{:?}", static_ref_y)
}