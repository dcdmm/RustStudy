pub mod outer_mod {
    pub mod inner_mod {
        // pub(crate): makes an item visible within the current crate.
        pub(crate) fn crate_visible_fn() {}

        // pub(super): makes an item visible to the parent module.
        pub(super) fn super_mod_visible_fn() {
            private_mod_visible_fn();
        }

        pub fn pub_mod_visible_fn() {}

        fn private_mod_visible_fn() {}
    }

    pub fn foo() {
        inner_mod::crate_visible_fn();
        inner_mod::super_mod_visible_fn();

        // error[E0603]: function `private_mod_visible_fn` is private
        // inner_mod::private_mod_visible_fn();
    }
}

fn bar() {
    // This function is still visible since we're in the same crate
    outer_mod::inner_mod::crate_visible_fn();

    // error[E0603]: function `super_mod_visible_fn` is private
    // outer_mod::inner_mod::super_mod_visible_fn(); // This function is no longer visible since we're outside of `outer_mod`

    outer_mod::foo();
}
