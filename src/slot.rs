mod character;
use character::Characters;

pub struct Slot;

impl Slot {
  pub fn new() {
    let character_name: String = Characters::new();
    println!("{}", character_name);
  }
}