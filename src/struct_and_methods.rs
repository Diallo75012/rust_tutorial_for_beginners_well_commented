// struct like Python Class
pub struct MangaKissa {
  pub room_number: u64,
  pub capacity: u64,
  pub toilet: u64,
  pub book_number: u64,
  pub computer_number: u64,
  // String like Python str
  // Rust has str also but it is like Python slice of `str[2:7]` or the full string but slice `[:]`
  pub comment: String,
  pub junko_present: bool,
}


// now like in Python Class we have methods
// we can implement methods to the struct using the `impl` keyword
impl MangaKissa {
  // Vector in Rust is like List in Python
  // let's make a method that turns the struct to a vector
  // `&` is a borrowed reference. this is dealing with how the memory is managed in Rust
  // the struct is the 'owner' of those vars and we 'borrow' those using `&` to use those.
  // not in Python
  pub fn to_vec(&self) -> Vec<String> {
    // use an existing macro to make the vector
    // like println!, when you see `!` it is a macro and we can use it
    // if we need some more custom stuff we need to do some more implementation and import some packages, but we are going to keep it simple
    vec![
      self.room_number.to_string(),
      self.capacity.to_string(),
      self.toilet.to_string(),
      self.book_number.to_string(),
      self.computer_number.to_string(),
      self.comment.to_string(),
      self.junko_present.to_string(),
    ]
  }

  // let's implement some more functionalities to our struct
  pub fn calculate(&self) -> u64 {
    // implicitly return the result so no need `;`
    self.room_number * self.capacity
  }

  // on last method so we play with the `String`
  pub fn comment_more(&self) -> String {
    // `.clone()` is used here to be able have variable and add to it the sentence
    // so `clone()` will copy explicitly the variable and not its reference (byte)
    self.comment.clone() + " You are happy and you go.... HAPPY!"
  }
}


// now let's make a function that will use the struct and its methods
// to play a bit with it
pub fn reserve_manga_kissa() -> Vec<String> {
  // let's instantiate the struct and create an object
  let shibuya_manga_kissa_danger = MangaKissa {
    room_number: 354,
    capacity: 2,
    toilet: 1,
    book_number: 1984,
    computer_number: 98,
    comment: String::from("Saturday night reading books in Shibuya."),
    junko_present: true,
  };

  // let's calculate capacity
  let calculate_capacity = shibuya_manga_kissa_danger.calculate().to_string();

  // let's create a vector from the struct fields
  // by default everything is 'immutable'
  // as we are going to extend the vector we use keyword `mut` (mutable please!)
  let mut shibuya_manga_kissa_danger_vec = shibuya_manga_kissa_danger.to_vec();

  // let's now add more variables in our 'mutable' (extendable/truncable) vector
  // use keyword `push(<String>)` to add to our vector some values
  shibuya_manga_kissa_danger_vec.push(calculate_capacity);

  // let use our last method and push it to the vector
  let keep_commenting = shibuya_manga_kissa_danger.comment_more();
  shibuya_manga_kissa_danger_vec.push(keep_commenting); //<String>

  // let's implicitly return the vector so no `;`
  shibuya_manga_kissa_danger_vec
}
