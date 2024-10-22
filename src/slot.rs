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
  fn spin(slice: &[&str]) -> String {
    let mut rng: ThreadRng = thread_rng();
    slice.choose(&mut rng).unwrap().to_string()
  }
}