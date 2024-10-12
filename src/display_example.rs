// import the formatting library `fmt`
use std::fmt;

// let's use `Rectangle` example 
#[derive(Debug)]
pub struct Rectangle {
  pub width: u64,
  pub height: u64,
}

// now we are going to implement the `Display` trait to the custom struct
impl fmt::Display for Rectangle {
  // here `Formatter` is like a buffer to store the custom formatting
  // and returns to `stdout`
  // `Result` return `Ok()` or `Err()`
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{} x {}", self.width, self.height)
  }
}

fn return_a_string() -> String {
  return "Volume at 0.0000000000001, can't hear anything!".to_string();
}

// let's make our exportable `pub` function
pub fn display_is_implemented_to_rectangle() -> String {
  // we instantiate a `Rectangle`
  let rect = Rectangle {
    width: 89,
    height: 78,
  };

  println!("Print using Implementation: {}", rect.width);
  // let's now compare to how it would look like with `Debug`
  // but we will need to put the decorator `#[derive(Debug)]` to `Rectangle` struct
  println!("Print using `Debug` trait: {:?}", rect);
  return_a_string()
  //"Success in implementing custom `Display` to custom struct!".to_string()
}








