// PS1="\[\033[01;32m\]Creditizens_Rusting_life:)\[\033[00m\]\[\033[01;34m\]->\[\033[00m\] "
// from the `main` function the compiler compiles ;)
// for comments : // or /**/
// use keyword `mod` to reference the module imported
/*
mod ev_num;

mod fibo;
mod struct_and_methods;
mod enum_types;
use crate::enum_types::MangaKissaIssue;
mod display_example;
mod error_handling;
mod options;
use options::Rectangle;
mod strings;
mod heap_the_stack;
mod loops;
*/

use std::any::Any;
mod boxes;
use crate::boxes::MangaKissaZone;

fn main() {

  // example 1: print enum
  let vec_box_from_enum = vec![
    MangaKissaZone::Attendance(3090),
    MangaKissaZone::Comment("Sooo Many People!".to_string()),
    MangaKissaZone::Temperature(31.9),
  ];
  println!("#########################################");
  boxes::print_manga_kissa_zone_debug_mode(vec_box_from_enum);
  println!("#########################################\n");

  // example 2: downcasting values from `Box`
  // here when creating the `Vec`
  // because we have to put elements in this Array `[elem1, elem2, ...]`
  // Rust wants all `elem` to be the same type like `[T;N]` for `Array` as we have seen in previous video
  // therefore, we will use then `as Box<dyn Any>`
  // Because elements have different types (String, i32, f64),
  // we must explicitly tell Rust that they are all boxed as Box<dyn Any>
  // We are `coercing` each boxed value. (inverse of `infer` types)
  // if we don't `tell the type = convert it here using `as Box<dyn Any>` we will get an error 
  let vec_box1: Vec<Box<dyn Any>> = Vec::from([
    Box::new("Castella Japan".to_string()) as Box<dyn Any>,
    Box::new(3_089_345) as Box<dyn Any>,
    Box::new(32.9) as Box<dyn Any>,
  ]);
  // Here we pass in `&mut Vec<Box<dyn Any>>`. As the type is know in the function
  // and the `Vec` exists so it is already initalized, inside the function
  // no need to convert the values pushed inside the vector: no need `as Box<dyn Any>`
  // from here rust will `infer` the types or automatically `coerce` those types to `Box<dyn Any>`
  boxes::box_simple_downcasting(vec_box1);
  
  // example 3: Downcasting and mutating using reference `&mut` to make it more fun
  println!("{:?}", boxes::box_downcasting_in_yoyogi(
      &mut Vec::from(
        [
         Box::new("I am in Starbuck Shibuya Watching At People Crossing And Their Different Styles.".to_string()) as Box<dyn Any>,
         Box::new(109) as Box<dyn Any>,
         Box::new(2007) as Box<dyn Any>, 
         Box::new(20.4) as Box<dyn Any>,
        ]
      )
  ));

  // example 4: using an `enum` as `Box` Type
  let shibuya_manga_kissa_level_7: Box<dyn Any> = Box::new(
    MangaKissaZone::Comment("A normal day reading books and whatching Naruto!".to_string()),
  ) as Box<dyn Any>;
  boxes::custom_enum_box_check(shibuya_manga_kissa_level_7);

}


  /*

  // BUUUuuuuut!
  println!("error if you omit this ';'!");
  // last one so implicitly runs and no need `;`
  println!("Shibuya, MangaKissa!");
  // `::` like `.` in Python `<module.func>`
  println!("{}", ev_num::even_number(13));
  // use `:?` when print is not implemented and can;t be formatted
  println!("{:?}", fibo::range_fibo(15));
  println!("{:?}", struct_and_methods::reserve_manga_kissa());
  // use of enum variants
  println!("{:?}", enum_types::issues_at_shibuya_action(
    &MangaKissaIssue::OneIssue("Naruto without any sound!".to_string())
    )
  );
  println!("{:?}", enum_types::issues_at_shibuya_action(
    &MangaKissaIssue::ListOfNumbers([109, 75012, 007].to_vec())
    )
  );
  println!("{:?}", enum_types::issues_at_shibuya_action(
    &MangaKissaIssue::IssueRanking(109)
    )
  );
  // Display implementation
  println!(
    // no `Debug` trait used (which is alrady implemented in t>
    "{}",
    display_example::display_is_implemented_to_rectangle()
  )


  */
