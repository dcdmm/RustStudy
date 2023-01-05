mod front_of_house;

pub use front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    front_of_house::serving::serving::take_order();
    front_of_house::serving::serving::serve_order();
    front_of_house::serving::serving::take_payment();
}
