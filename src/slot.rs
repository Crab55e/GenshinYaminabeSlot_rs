use rand::rngs::ThreadRng;
use rand::thread_rng;
use rand::seq::SliceRandom;

mod character;
use character::Characters;
mod action;
use action::Actions;
mod description;
use description::Descriptions;

pub struct Slot;

impl Slot {
  pub fn new() -> String {
    let character: String = Characters::new();
    let description: String = Descriptions::new();
    let action: String = Actions::new();
    format!("{}が{}{}", character, description, action)
  }
  fn spin(slice: &[&str]) -> String {
    let mut rng: ThreadRng = thread_rng();
    slice.choose(&mut rng).unwrap().to_string()
  }
}