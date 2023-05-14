// std::boxed::Box

/*
常见方法:
new
 */

#[test]
fn new_fn() {
    /*
    pub fn new(x: T) -> Box<T, Global>
        Allocates memory on the heap and then places x into it.

        This doesn’t actually allocate if T is zero-sized.
     */
    let _five = Box::new(5);
}