use rand::rngs::ThreadRng;
use rand::thread_rng;
use rand::seq::SliceRandom;

mod character;
use character::Characters;
mod action;

pub struct Slot;

impl Slot {
  pub fn new() {
    let character_name: String = Characters::new();
    println!("{}", character_name);
  }
  fn spin(slice: &[&str]) -> String {
    let mut rng: ThreadRng = thread_rng();
    slice.choose(&mut rng).unwrap().to_string()
  }
}