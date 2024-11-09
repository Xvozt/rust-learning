use crate::front_of_house::hosting; //idiomatic way to use parent module of needed func
use crate::front_of_house::hosting::add_to_waitlist;
//not good because isn't clear or its a local func or brought to the scope
use std::collections::HashMap; // but using structs its idiomatic to bring full path
use std::fmt::Result;
use std::io::Result as IOResult; // this is one way to bring 2 items to scope with the same way other way is to bring parent modules fmt and io and use with parent name ot disntiguish

//use std::{cmp::Ordering, io}; // nested imports
//use std::io::{self, Write}; // nested path with self, this line brings std::io and std::io::Write
// use std::collections::*; // glob operator

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
        fn seat_at_table() {}
    }
    mod serving {
        fn take_order() {}
        fn serve_order() {}
        fn take_payment() {}
    }
}

// pub use crate::front_of_house::hosting; // re-exporting module

// pub fn eat_at_cafe() {
//     hosting::add_to_waitlist();
// }

// fn function() -> Result {};
// fn function2() -> IOResult<()> {};

mod customer {
    pub fn eat_at_restaurant() {
        // crate::front_of_house::hosting::add_to_waitlist();
        // front_of_house::hosting::add_to_waitlist();
        // hosting::add_to_waitlist();
        // // use at root doesnt work, so we need to use super:: to come to parent mod and then use will work
        super::hosting::add_to_waitlist();

        let mut meal = super::back_of_house::Breakfast::summer("Rye");
        meal.toast = String::from("Wheat");
        println!("I'd like {} toast please", meal.toast);

        let order1 = super::back_of_house::Appetizer::Soup;
        let order2 = super::back_of_house::Appetizer::Salad;
    }
}

pub fn bad_practice() {
    add_to_waitlist(); // it's not clear whether this func is local or brought to the scope
    let mut map = HashMap::new();
    map.insert(String::from("some"), 5);
}

fn deliver_oder() {}

mod back_of_house {
    // if a struct is public it doesnt mean that all the fields are pulbic.
    pub struct Breakfast {
        pub toast: String, // we need to explicitly make the fields public
        seasonal_fruit: String,
    }

    impl Breakfast {
        // to access a struct with some private fields we need a "constuctor-like" func to initialize the instance of struct
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
    // on the other hand making a enum public will make all the variants public
    pub enum Appetizer {
        Soup,
        Salad,
    }

    fn fix_incorrect_order() {
        cook_order();
        super::deliver_oder();
    }

    fn cook_order() {}
}
