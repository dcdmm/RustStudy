#[allow(unused)]
pub mod back_of_house {
    // if we make an enum public, all of its variants are then public.
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

#[allow(unused)]
pub fn eat_at_restaurant() {
    let order1 = back_of_house::Appetizer::Soup;
    let order2 = back_of_house::Appetizer::Salad;
}