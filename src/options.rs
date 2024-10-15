// we are going to use here a series a functions to represent
// the different examples

use std::fs;
use std::io;


// 1. General example: what is an Option
#[allow(dead_code)]
pub fn use_option_at_shibuya(sentence: String) -> 
  Option<u64> {
    for (index, letter) in sentence.chars().enumerate() {
      if letter == 'J' {
        return Some(index as u64);
    }
  }
  None
}


// 1.  misuse of Option / Error case
#[allow(dead_code)]
pub fn read_file(path: &str) -> Result<String, io::Error> {
  // file reading might fail due to IO error
  fs::read_to_string(path)
}


// 2. misuse nested Option<Option<T>>
#[allow(dead_code)]
pub fn get_value() -> Option<u64> {
  Some(115)
}

// 3. misuese Option used when Default value is better
// here `usize` is also a type of integer but will detect the underlying system if 32 bits will be u32 and if 64 bits will be u64
// it is `u`size so `u32/64` depending on your underlying system
// indexes are also in rust expected to be usizes 
#[allow(dead_code)]
pub fn count_items(items: &[u64]) -> usize {
  items.len() // returns 0 if the slice is empty
}


// 4. more complex here: custom struct with Option to wrap it off
// we are not going to implement formatting
// therefore will use here the debug trait
#[derive(Debug)]
pub struct Rectangle {
  // otherwise if no `pub` for fields in `main.rs`
  // use `use crate::options::Rectangle;`
  pub width: u64,
  pub height: u64,
}

pub fn validate_rectangle(rect: Rectangle) -> Option<Rectangle> {
  if (4..=10).contains(&rect.width) && (20..=30).contains(&rect.height) {
    Some(rect)
  } else {
    None
  }
}




