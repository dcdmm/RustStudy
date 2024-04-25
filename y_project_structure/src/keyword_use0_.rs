// use关键字

/*
Import or rename items from other crates or modules.

Usually a use keyword is used to shorten the path required to refer to a module item. The keyword may appear in modules, blocks and even functions, usually at the top.
*/

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }
}

// mod hosting {
//     pub fn add_to_waitlist() {}

//     fn seat_at_table() {}
// }
use front_of_house::hosting; // 与上等价

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}

pub mod customer {
    // mod hosting {
    //     pub fn add_to_waitlist() {}

    //     fn seat_at_table() {}
    // }
    use super::front_of_house::hosting; // 与上等价

    pub fn eat_at_restaurant() {
        hosting::add_to_waitlist();
    }
}
