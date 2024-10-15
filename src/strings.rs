// we are going to use one public function
// and put different example in one by one
// we are going to return a 'flexible' tuple
#[allow(dead_code)]
fn print_type_of<T: ?Sized>(_: &T) {
    println!("{}", std::any::type_name::<T>());
}

pub fn string_or_str() -> (String, String) {
  /*
  // example 1: truncate, replace, slice
  // truncate
                          //0123456789.....
  let mut s = String::from("Crowded Shibuya");
  println!("s: {:?}", s);
  let s_truncated = s.truncate(7);
  println!("s_truncated: {:?}", s_truncated);

  // replace
  // we expect `s` to be equal to `Crowded`
  let s2_replaced = s.replace("Crowd", "Load");
  println!("s2_replaced: {}", s2_replaced);

  // slice
  // here we expect `s2_replaced` to be `Loaded`
  let s3 = s2_replaced.clone();
  println!("s3: {}", s3);
  let s3_sliced = &s3[0..=1];
  // we expect `s3_sliced` to be equal to `Lo`
  println!("s3_sliced: {:?}", s3_sliced);
  print_type_of(s3_sliced);
  // returning x3 `String` therefore adapt the function number `String` returns
  (s, s2_replaced, s3_sliced.to_string())


  // example 2: `len()` and `.chars().count()`
                                 // 0123456789
  let queue_at_strabucks_shibuya = "20 people".to_string();
  let length = queue_at_strabucks_shibuya.len();
  let lenght_type = print_type_of(&length);
  println!("{:?}", lenght_type);

  let characters_in_queue = queue_at_strabucks_shibuya.
chars().count();
  let characters_queue_type = print_type_of(&characters_in_queue);

  println!("chars().count() type: {:?}; while chars() only is type: {:?}", characters_queue_type, queue_at_strabucks_shibuya.chars()); 

  // returning only a tuple of two for this example so need to adapt the functing return number of `String`
  (length.to_string(), characters_in_queue.to_string())


  // example 3: `''` vs `""` and iterator with `next()`

  let mut purikura = String::from("Rejeuvenate through Purikura...");
  // character = single quote, we append to `String` with `.push()`
  purikura.push('-');
  purikura.push('>');
  println!("Purikura is..:{}", purikura);

  // " " = `&str` (`String` slice), we append to `String` with `.push_str()`
  purikura.push_str(" Suuuuuppaaaaa photos!!!");    
  println!("Puuuuuurika is now: {}", purikura);
 
  // i will instead show what is `.next()` doing and how the iterators work in general
  let purikura_clone = purikura.clone();
  let mut word_iterator = purikura_clone.split_whitespace();
  let word_iterator_type = print_type_of(&word_iterator);
  println!("word_iterator is: {:?} and it's type is {:?}", word_iterator, word_iterator_type);
  
  let next_word = word_iterator.next();
  println!("next_word is {:?} and type is {:?}", next_word, print_type_of(&next_word));

  let second_word = match word_iterator.next() {
    Some(word) => word.to_string(),
    None => String::new(),
  };
  
  // here returning tuple with 2 variables, make sure to change the number of `String` returned
  (purikura, second_word)
  */


  // Interesting `String` manipulations

  let free_mangakissa_drinks = String::from("hot and cold coffee, strawberry milk, cola, water, soup and more...");

  // use of `format!()` to copy `String`
  // you can also use it to concatenate `String` but here we won't
  let some_drinks_in_mangakissa	= format!("{}", free_mangakissa_drinks);

  // now lets add some interesting extra checks on the memory location
  // of those `String`: original versus copy using `format!()`
  assert_eq!(&free_mangakissa_drinks as *const String, &some_drinks_in_mangakissa as *const String); 

  (free_mangakissa_drinks, some_drinks_in_mangakissa)

}





