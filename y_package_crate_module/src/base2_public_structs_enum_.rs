pub mod public_struct {
    /*
    If we use pub before a struct definition, we make the struct public, but the struct’s fields will still be private. We can make each field public or not on a case-by-case basis.
     */
    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    /*
    Because public_struct::Breakfast has a private field, the struct needs to provide a public associated function that constructs an instance of Breakfast. If Breakfast didn’t have such a function, we couldn’t create an instance of Breakfast in eat_at_restaurant_struct because we couldn’t set the value of the private seasonal_fruit field in eat_at_restaurant_struct.
     */
    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

pub fn eat_at_restaurant_struct() {
    // // error[E0451]: field `seasonal_fruit` of struct `Breakfast` is private
    // let mf = public_struct::Breakfast {
    //     toast: String::from("Wheat"),
    //     seasonal_fruit: String::from("blueberries"),
    // };

    let mut meal = public_struct::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("{}", meal.toast);

    // error[E0616]: field `seasonal_fruit` of struct `Breakfast` is private
    // meal.seasonal_fruit = String::from("blueberries");
}

pub mod public_enum {
    // if we make an enum public, all of its variants are then public.
    pub enum Appetizer {
        Soup,
        Salad,
    }
}

pub fn eat_at_restaurant_enum() {
    let order1 = public_enum::Appetizer::Soup;
    let order2 = public_enum::Appetizer::Salad;
}
