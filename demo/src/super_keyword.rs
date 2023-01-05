fn deliver_order() {}

mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        crate::deliver_order(); 
        // We can construct relative paths that begin in the parent module, rather than the current module or the crate root, by using super at the start of the path.
        // The fix_incorrect_order function is in the back_of_house module, so we can use super to go to the parent module of back_of_house, which in this case is crate, the root
        super::deliver_order(); // 与上等价
    }

    fn cook_order() {}
}
