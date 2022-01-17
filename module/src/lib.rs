mod front_of_house;
mod back_of_house;

pub use crate::back_of_house::BreakFast;

pub fn eat_at_summer_restaruant() {
    let mut meal = BreakFast::summer("Rye");
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);
}

pub use crate::front_of_house::hositing;

pub fn eat_at_restaurant() {
    hositing::add_to_waitlist();
    hositing::add_to_waitlist();
}