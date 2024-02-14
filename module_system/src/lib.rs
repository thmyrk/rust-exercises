mod front_of_house {
    fn toggle_customer_presence() {}

    pub mod hosting {
        pub fn add_to_waitlist() {
            super::toggle_customer_presence();
            seat_at_table();
        }

        fn seat_at_table() {}
    }

    pub mod serving {
        pub fn take_order() {
            call_waiter();
        }

        pub fn serve_order() {
            call_waiter();
        }

        pub fn take_payment() {
            call_waiter();
            crate::front_of_house::toggle_customer_presence();
        }

        fn call_waiter() {}
    }
}

mod back_of_house {
    pub struct Breakfast {
        toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }

        pub fn description(&self) -> String {
            String::from(format!("{0} toast with {1}", self.toast, self.seasonal_fruit))
        }
    }

    pub enum Appetizer {
        Soup,
        Salad,
    }

    impl Appetizer {
        pub fn name(&self) -> &str {
            match self {
                Appetizer::Soup => "Soup",
                Appetizer::Salad => "Salad",
            }
        }
    }
}

pub fn eat_at_restaurant() {
    front_of_house::hosting::add_to_waitlist();
    front_of_house::serving::take_order();

    let meal = back_of_house::Breakfast::summer("Wheat");
    println!("Meal given is: {0}", meal.description());

    let appetizer1 = back_of_house::Appetizer::Soup;
    let appetizer2 = back_of_house::Appetizer::Salad;
    println!("Appetizers given are: {0} and {1}", appetizer1.name(), appetizer2.name());

    front_of_house::serving::serve_order();
    front_of_house::serving::take_payment();
}
