pub struct BreakFast {
    pub toast: String,
    seasion_fruit: String
}

impl BreakFast {
    pub fn summer(toast: &str)-> BreakFast {
        BreakFast {
            toast: String::from(toast),
            seasion_fruit: String::from("peaches")
        }
    }
}