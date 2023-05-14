// Bringing Paths into Scope with the use Keyword

#[allow(unused)]
pub mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

/*
When we bring a name into scope with the use keyword, the name available in the new scope is private.
To enable the code that calls our code to refer to that name as if it had been defined in that code’s scope, we can combine pub and use.
This technique is called re-exporting because we’re bringing an item into scope but also making that item available for others to bring into their scope.
 */
// main函数中调用add_to_waitlist:`use use_keyword2_::hosting;`
pub use crate::use_keyword2_::front_of_house::hosting;

// 13行去掉pub关键字,即:use crate::use_keyword2_::front_of_house::hosting;
// main函数中调用add_to_waitlist:use use_keyword2_::front_of_house::hosting::add_to_waitlist;