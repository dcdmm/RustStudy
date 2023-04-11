/*
The module tree for the code in Listing 1
crate
 └── front_of_house
     ├── hosting
     │   ├── add_to_waitlist
     │   └── seat_at_table
     └── serving
         ├── take_order
         ├── serve_order
         └── take_payment
 */

// Listing 1: A front_of_house module containing other modules that then contain functions
#[allow(unused)]
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    /*
    Adding the pub keyword in front of mod hosting makes the module public.
    With this change, if we can access front_of_house, we can access serving.
    But the contents of serving are still private; making the module public doesn’t make its contents public.
    The pub keyword on a module only lets code in its ancestor modules refer to it, not access its inner code.
    Because modules are containers, there’s not much we can do by only making the module public;
    we need to go further and choose to make one or more of the items within the module public as well.

    Items in a parent module can’t use the private items inside child modules, but items in child modules can use the items in their ancestor modules. This is because child modules wrap and hide their implementation details, but the child modules can see the context in which they’re defined.
    To continue with our metaphor, think of the privacy rules as being like the back office of a restaurant: what goes on in there is private to restaurant customers, but office managers can see and do everything in the restaurant they operate.
     */
    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}


/*
A path can take two forms:

* An absolute path is the full path starting from a crate root; for code from an external crate, the absolute path begins with the crate name, and for code from the current crate, it starts with the literal crate.
* A relative path starts from the current module and uses self, super, or an identifier in the current module.

Both absolute and relative paths are followed by one or more identifiers separated by double colons (::).
 */
#[allow(unused)]
pub fn eat_at_restaurant() {
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    front_of_house::hosting::add_to_waitlist();
}

#[allow(unused)]
fn deliver_order() {}

#[allow(unused)]
mod back_of_house {
    fn fix_incorrect_order() {
        cook_order();
        /*
        We can construct relative paths that begin in the parent module, rather than the current module or the crate root, by using super at the start of the path.
        This is like starting a filesystem path with the .. syntax.
        Using super allows us to reference an item that we know is in the parent module, which can make rearranging the module tree easier when the module is closely related to the parent, but the parent might be moved elsewhere in the module tree someday.
         */
        super::deliver_order();
    }

    fn cook_order() {}
}
