use std::fmt;

#[derive(Debug)]
pub struct Rectangle {
  pub width: u64,
  pub height: u64,
}

impl fmt::Display for Rectangle {
  // this is using a reference so borrowing
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{} x {}", self.width, self.height)
    // Can use debut as a fallback
    // write!(f, "{:?} x {:?}", self.width, self.height)
  }
}

pub fn display_is_implemented_to_rectangle() {
  let rect = Rectangle {
    width: 30,
    height: 60
  };
  println!("Print using Implementation: {}", rect);
  // the Debug `{:?}` is borrowing and not moving ownership
  println!("Print using Debug already in print macro implemting its own Display: {:?}", rect);
  // that is why we cans till access `rect` variable instantiating the struct `Rectangle`: here we access the fields
  println!("We still have access to `rect` instance of Rectangle struct: '{:?} x {:?}'. do you know why?", rect.height, rect.width);
}
