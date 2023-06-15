pub struct Breakfast {
  pub toast: String,
  fruit: String
}

impl Breakfast {
  pub fn choose_breakfase(toast: &str) -> Breakfast {
    Breakfast { toast: String::from(toast), fruit: String::from("pear") }
  }
}