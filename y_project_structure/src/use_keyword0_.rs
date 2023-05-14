// Bringing Paths into Scope with the use Keyword

#[allow(unused)]
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }
}

// Paths brought into scope with use also check privacy, like any other paths.
// 报错:error[E0603]: function `seat_at_table` is private
// use front_of_house::hosting::seat_at_table;
#[allow(unused)]
// use crate::use_keyword0_::front_of_house::hosting; // 绝对路径
use front_of_house::hosting; // 相对路径;与上等价

#[allow(unused)]
pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}

/*
Note that use only creates the shortcut for the particular scope in which the use occurs.
Listing 1 moves the eat_at_restaurant function into a new child module named customer, which is then a different scope than the use statement, so the function body won’t compile:
 */
// Listing 1: A use statement only applies in the scope it’s in
#[allow(unused)]
pub mod customer {
    // use crate::use_keyword::front_of_house::hosting; // 绝对路径
    use super::front_of_house::hosting; // 相对路径;与上等价

    pub fn eat_at_restaurant() {
        hosting::add_to_waitlist(); // 从第32行`use super::front_of_house::hosting;`调用,而不是从17行`use front_of_house::hosting;`调用
    }
}