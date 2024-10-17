// we are going to use one public function
// and put different examples in one by one
// we are going to return a 'flexible' tuple (just change the nbr of types returned and sue probably `String`)

// ignore this for now , it is just a helper function to check types
fn print_type_of<T: ?Sized>(_: &T) {
  println!("{}", std::any::type_name::<T>());
}

pub fn string_or_str() -> (String, String) {

  /*
  // example 1: truncate, replace, slice

  // truncate
                        //  0123456789.....
  let mut s = String::from("Crowded Shibuya");
  println!("s: {:?}", s);
  let s_truncated = s.truncate(7);
  // this one is interesting and we will explain on terminal build
  // returns `()`... why? because it is truncating `s` "in place"
  // 'in place' mean that `s_truncated` is doing an operation on `s`
  // but not holding any variable (it is not like in Python creating a new variable)
  println!("s_truncated: {:?}", s_truncated);

  // replace
  // we expect `s` to be equal to `Crowded`
  let s2_replaced = s.replace("Crowd", "Load");
  println!("s2_replaced: {}", s2_replaced);

  // slice
                                                  // 01234
  // here we can expect `s2_replaced` to be equal to `Load`
  let s3 = s2_replaced.clone();
  println!("s3: {}", s3);
  // s3_sliced is an `&str`
  let s3_sliced = &s3[0..=1];
  // therefore here s3_sliced should be equal to `Lo`
  println!("s3_sliced: {:?}", s3_sliced);
  print_type_of(s3_sliced);

  // this is how many `Stirng` are returned: x3
  // therefore, do not forget to change the function number of returned values
  (s, s2_replaced, s3_sliced.to_string())


  // example 2: `.len()` and `.chars().count()`

  let queue_at_starbucks_shibuya = "20 people".to_string();
  let length = queue_at_starbucks_shibuya.len();
  print_type_of(&length);

  let characters_in_queue = queue_at_starbucks_shibuya.chars().count();
  let characters_in_queue_type = print_type_of(&characters_in_queue);

  println!("chars().count() type: {:?}; while chars() only type is: {:?}", characters_in_queue_type, queue_at_starbucks_shibuya.chars());

  // do not forget to change the number of returned `String`
  (length.to_string(), characters_in_queue.to_string())


  // example 3: `""` vs `''` and iterator with `.next()`: interesting!

  let mut purikura = String::from("Rejeuvenate through Purika...");
  // character = single quotes, we append to `String` with `.push()`
  purikura.push('-');
  purikura.push('>');
  println!("Purikura is..: {}", purikura);

  // " " = `&str` (`String` slice), we append to `String` with `.push_str()`
  purikura.push_str(" Suuuuupppaaaaaa photos!!!");
  println!("Puuuuuuurikura is now: {}", purikura);

  // I will show what is `.next()` doing and how iterators work in general
  // we could have used `for word in purikura_clone.split_whitespace()`
  // and iterate through it normally but here we want to show what is happening in steps and choose to not iterate over
  // every word, `.next()` will help to navigate that string word by word
  let purikura_clone = purikura.clone();
  let mut word_iterator = purikura_clone.split_whitespace();
  let word_iterator_type = print_type_of(&word_iterator);
  println!("word_iterator is: {:?} and its type is: {:?}", word_iterator, word_iterator_type);

  let next_word = word_iterator.next();
  println!("next_word is {:?}  and type is {:?}", next_word, print_type_of(&next_word));


  // `.expect()` here is to panic! if no value is found meaning iterator arrived at the end
  // but i use it as well to be able to get the value and convert it to a `String`
  // we will see that next type is `Option` therefore we need to get the `Some()` side value and `expect()` 'unwraps' it
  // instead of `expect()` we could have uses `match` or `if let`
  // like that:
  // `word_itertor.next()` is of type `Option` (returns `Some()` or `None`)
  //let final_output = match word_iterator.next() {
   // Some(word) => word.to_string(), / could get a `String` back like that
    //None => String::new(), / return empty `String` to not `panic!` 
  //}
  (purikura, word_iterator.next().expect("An Error Occured While Trying To Get Next Word...").to_string())
  */

  // example 4: the last one 
  let free_mangakissa_drinks = String::from(
  "hot and cold coffee, strawberry milk, cola, water, soup and more..."
  );

  // we are going to use `format!` and play with it to understand some nuances
  // `format!` can also be used to concatenate `String` together but here we won't do that (too easy)
  let some_drinks_in_mangakissa = format!("{}", free_mangakissa_drinks);

  // now let's add some interesting extra checks on the memory location
  // `format!` is supposed to be a copy of `String`
  // we will check that by checking on the memory locations
  // we will use like in Python code testing `assertions`
  // we assert/suppose that the two are the same because they are copies, I mean of is copy of the other
  // `as` is to convert/cast `&str` to `String`
  // `*const` is to get the value of the memory location of that actual `String`
  // so here `format!` makes a copy and creates a new `String` like in Python
  assert_eq!(
    &free_mangakissa_drinks as *const String, //  left: 0x7ffd3d6cf5d8
    &some_drinks_in_mangakissa as *const String // right: 0x7ffd3d6cf5f0
  );



  (free_mangakissa_drinks, some_drinks_in_mangakissa)
}







