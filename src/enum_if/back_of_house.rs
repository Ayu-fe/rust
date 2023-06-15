pub mod host;

pub fn eat_breakfast() {
  let mut breakfast = host::Breakfast::choose_breakfase("cheese");
  breakfast.toast = String::from("bitch");
}