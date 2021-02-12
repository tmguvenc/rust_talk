use std::collections::HashMap;

fn main() {
  let mut m: HashMap<u8, String> = HashMap::new();
  m.insert(5, "rust".to_owned());
  let val = m.get(&6);

  println!("{:?}", val);
}
