// In Rust, all items (functions, methods, structs, enums, modules, and constants) are private to parent modules by default. 
// Items in a parent module can’t use the private items inside child modules, but items in child modules can use the items in their ancestor modules. 
mod front_of_house {sdf
    // Exposing Paths with the pub Keyword
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

// While front_of_house isn’t public, because the eat_at_restaurant function is defined in the same module as front_of_house (that is, eat_at_restaurant and front_of_house are siblings), we can refer to front_of_house from eat_at_restaurant. 
pub fn eat_at_restaurant() {
    // Both absolute and relative paths are followed by one or more identifiers separated by double colons (::).

    // Absolute path
    // An absolute path is the full path starting from a crate root; for code from an external crate, the absolute path begins with the crate name, and for code from the current crate, it starts with the literal crate.
    crate::front_of_house::hosting::add_to_waitlist();

    // Relative path
    // A relative path starts from the current module and uses self, super, or an identifier in the current module.
    front_of_house::hosting::add_to_waitlist();
}
