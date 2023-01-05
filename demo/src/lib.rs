// mod front_of_house {
//     pub mod hosting{
//         pub fn add_to_waitlist() {}
//     }
//     pub mod serving{
//         pub mod serving {
//             pub fn take_order() {}
        
//             pub fn serve_order() {}
        
//             pub fn take_payment() {}
//         }
//     }

//     pub fn sleep() {}    
// }
mod front_of_house;  // 与上等价

pub use front_of_house::sleep;
pub use front_of_house::hosting;

pub fn eat_at_restaurant() {
    sleep();
    hosting::add_to_waitlist();
    front_of_house::serving::serving::take_order();
    front_of_house::serving::serving::serve_order();
    front_of_house::serving::serving::take_payment();
}
