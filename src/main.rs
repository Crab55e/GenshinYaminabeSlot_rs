mod slot;
use slot::Slot;

fn main() {
  let ans_slot: String = Slot::new();
  println!("{}", ans_slot);
}
