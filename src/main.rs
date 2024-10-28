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
mod boxes;
//#[allow(unused_imports)]
use crate::boxes::MangaKissaZone;
use std::any::Any;
mod generic_manga_kissa;
mod trait_shibuya_109;
mod trait_shibuya_109;
mod mistralai;
use mistralai::mistral;
mod hidden_vars; // folder
use hidden_vars::read_hidden_vars; // module in folder
*/

mod junko_lifetimes;

fn main() {
  //junko_lifetimes::lifetimes_different_scope()
  //junko_lifetimes::multiple_lifetimes_to_struct()
  junko_lifetimes::multiple_lifetimes_to_struct_scope_limited()
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
  // Box
  // example 1: print enum
  let vec_box_from_enum = vec![
    // i put those in different order just for you
    // to know that the order doesn't matter
    MangaKissaZone::Attendance(3090),
    MangaKissaZone::Comment("Sooo Many People".to_string()),
    MangaKissaZone::Temperature(31.9),
  ];
  println!("################################");
  boxes::print_manga_kissa_zone_debug_mode(
    vec_box_from_enum
  );
  println!("################################");

  // example 2: downcasting values from Real `Box`
  // we create a `Vec` and use `[elem, elem2, ...]`
  // which is making a `Vector` from an `Array`
  // Rust wants all `elem` to be same type like in `[T;N]` for `Array` as we have seen in previous video
  // therefore, we will use then `as Box<dyn Any>`
  // because elements have differet types (String, f64, i32)
  // we must explicitly tell Rust that they are all boxes as `Box<dyn Any>` (therefore, same type in the `Array` to transform to a `Vec`)
  // we are "coercing" each boxed value type. (inverse of `infer` types)
  // if we don't tell the type = convert it here using `Box<dyn Any>`, we will get an error at compile time
  let vec_box_castella: Vec<Box<dyn Any>> = Vec::from([
    Box::new("Castella Japan".to_string()) as Box<dyn Any>,
    Box::new(3_089_345) as Box<dyn Any>,
    Box::new(32.9) as Box<dyn Any>,
  ]);

  boxes::box_simple_downcasting(vec_box_castella);

  // example 3: downcasting and mutating using `&mut` to make it more fun
  // here we pass in `&mut Vec<Box<dyn Any>>`
  // as the type is know in the function
  // and the `Vec` exists so it is already initialized
  // inside the function no need to convert the values
  // when pushing those inside the vector
  // no need `as Box<dyn Any>`
  // from inside the function Rust will "infer"
  // therefore automatically "coerce" thos etypes to `Box<dyn Any>`
  println!(
     "{:?}",
      boxes::box_downcasting_in_yoyogi(
        &mut Vec::from([
          Box::new(
            1
          ) as Box<dyn Any>,
          Box::new(
            "I am in Starbucks Shibuya Watching at People Crossing and Their Different Styles Entertains Me."
            .to_string()
          ) as Box<dyn Any>,
          Box::new(
            109
          // so here we coearce the type manually
          // and there Rust will 'infer' automatically
          ) as Box<dyn Any>,
          Box::new(
            2007
          ) as Box<dyn Any>,
          Box::new(
            20.4
          ) as Box<dyn Any>,
       ])
      )
  )
 

  // example 4: using an `enum` as `Box` type
  // the `Enum` will live in the Heap
  // and the String Type will be moved there
  // if you need to have less stuff in the Stack
  // or think that it is going to grow and "overflow"
  // You can use `Box` for that, with `Enum` it becomes powerful
  let shibuya_manga_kissa_level_7: Box<dyn Any> = Box::new(
    MangaKissaZone::Comment(
      "A normal day reading books while wathcing Naruto without sound!".to_string(),
    )
  ) as Box<dyn Any>;
  
  boxes::custom_enum_box_check(shibuya_manga_kissa_level_7)

  */
