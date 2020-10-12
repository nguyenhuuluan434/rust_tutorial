mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}

        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}

        pub fn serve_order() {}

        fn take_payment() {}
    }

    pub mod back_of_house {
        fn fix_incorrect_order() {
            cook_order();
            super::serving::serve_order();
        }

        fn cook_order() {}

        pub struct Breakfast {
            pub toast: String,
            seasonal_fruit: String,
        }

        impl Breakfast {
            pub fn summer(toast: &str) -> Breakfast {
                Breakfast {
                    toast: String::from(toast),
                    seasonal_fruit: String::from("peaches"),
                }
            }
        }

        pub enum Appetizer {
            Soup,
            Salad,
        }
    }

    pub fn eat_at_restaurant() {
        // Order a breakfast in the summer with Rye toast
        let mut meal = back_of_house::Breakfast::summer("Rye");
        // Change our mind about what bread we'd like
        meal.toast = String::from("Wheat");
        println!("I'd like {} toast please", meal.toast);

        // The next line won't compile if we uncomment it; we're not allowed
        // to see or modify the seasonal fruit that comes with the meal
        //meal.seasonal_fruit = String::from("blueberries");
    }
}

pub fn eat_at_restaurant() {
    //absolute path what start by crate that is crate name
    crate::front_of_house::hosting::add_to_waitlist();
    //relative path what
    front_of_house::hosting::add_to_waitlist();
    let order1 = front_of_house::back_of_house::Appetizer::Soup;
    let order2 = front_of_house::back_of_house::Appetizer::Salad;
}


use crate::front_of_house::hosting;

pub fn eat_at_restaurant_use_keyword_version() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}
