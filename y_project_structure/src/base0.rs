// If an item is private(默认), it may be accessed by the current module and its descendants.
mod front_of_house {
    /*
    Adding the pub keyword in front of mod hosting makes the module public. With this change, if we can access front_of_house, we can access hosting. But the contents of hosting are still private; making the module public doesn’t make its contents public. The pub keyword on a module only lets code in its ancestor modules refer to it, not access its inner code. Because modules are containers, there’s not much we can do by only making the module public; we need to go further and choose to make one or more of the items within the module public as well.
     */
    pub mod hosting {
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    /*
    A path can take two forms:
        * An absolute path is the full path starting from a crate root; for code from an external crate, the absolute path begins with the crate name, and for code from the current crate, it starts with the literal crate.
        * A relative path starts from the current module and uses self, super, or an identifier in the current module.

    Both absolute and relative paths are followed by one or more identifiers separated by double colons (::).
    */
    pub fn public_function() {
        crate::base0::front_of_house::hosting::add_to_waitlist(); // 绝对路径
        hosting::add_to_waitlist(); // 相对路径

        // error[E0603]: function `seat_at_table` is private
        // hosting::seat_at_table(); // 函数pulic_function位于模块hosting父模块(front_of_house)中

        println!("This is a public function.");
    }

    fn private_function() {
        println!("This is a private function.");
    }

    mod child {
        pub fn demonstrate_access() {
            /*
            We can construct relative paths that begin in the parent module, rather than the current module or the crate root, by using super at the start of the path. This is like starting a filesystem path with the .. syntax.
             */
            super::public_function();
            // 模块child是模块front_of_house的子模块
            super::private_function(); // 子模块访问私有项
        }
    }
}

pub fn eat_at_restaurant() {
    crate::base0::front_of_house::hosting::add_to_waitlist();

    front_of_house::hosting::add_to_waitlist();

    // error[E0603]: function `seat_at_table` is private
    // front_of_house::hosting::seat_at_table(); // 函数eat_at_restaurant位于模块hosting父模块(front_of_house)的父模块(base0)中
}

fn public_api() {}

mod other_module {
    fn my_method() {
        // 模块other_module、模块front_of_house、函数public_api都属于当前模块(base0)中
        super::front_of_house::hosting::add_to_waitlist(); // 同一模块中访问私有项
        super::public_api(); // 同一模块中访问私有项
    }
}