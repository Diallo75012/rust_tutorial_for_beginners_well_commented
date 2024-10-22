// now that we have seen several concepts,
// you won't be afraidto see me putting some decorators
// we are going to allow dead_code and activate debug for enums

use std::any::Any;


#[allow(dead_code)]
#[derive(Debug)]
pub enum MangaKissaZone {
  Attendance(u64),
  Temperature(f64),
  Comment(String),
}

/*
// example 1: print enum

// note here that this is a trick to allow a `Vector`,
// which is normally accepting only one `type`,
// to hold different types using `Enum`(Vec<MangaKissaZone>)
pub fn print_manga_kissa_zone_debug_mode(
  vec: Vec<MangaKissaZone>) {
    println!("Vector is: {:?}", vec)
}



// but that is not a real `Box` even if it can hold
// different types. When we talk about `Box` it is about "memory" allocation

// example 2: creating real `Box` and downcasting values

pub fn box_simple_downcasting(

  vec_simple_box: Vec<Box<dyn Any>>
  ) {

  // `Box<dyn Any>` returns an `Option` (Some or None)
  // retrieve values by downcasting each element type
  // `downcasting_ref` convert those values if needed to
  // this is possible because we have `Any` which helps to know type at "Runtime"
  // but this means that it adds more checks at "runtime" and can alter performance
  // also it can't be used everywhere just for some specific use cases if needed to perform some operation
  // using the valu behind the reference `Box`
  // useing `Box<dyn Any>` tool much will increase the chances of getting "runtime errors"
  // because of incorrect `Type` assumptions (Know What You Do! For You and For the Compiler as Well!)
  for elem in vec_simple_box.iter() {
    if let Some(val) = elem.downcast_ref::<i32>() {
      println!("Found i32: {}", val);
    } else if let Some(val) = elem.downcast_ref::<f64>() {
      println!("Found f64: {}", val);
    } else if let Some(val) = elem.downcast_ref::<String>() {
      println!("Found String: {}", val);
    } else {
      println!("Unknown type!");
    }
  }

}


// example 3: Downcasting and mutating using mutable reference `&mut` to make it more fun

// that is the type `Box<dyn Any>` used for boxes as you have seen in the example before...
pub fn box_downcasting_in_yoyogi(
  // here whatever in inside `Vec` will be on the `Heap` memory
  // while `Box` reference stays on the `Stack`
  // referencing the types moved to the heap (if normally stack (like String, u/i...))
  // or native to heap side of memory
  // here `dyn` for `Dynamic` will permit to figure out `Types` at "Runtime"
  // learn here about the word `Polymorphism`:
  // '''Having a `Trait` interface for example being implemented
  // by several different `struct` 
  // (`Struct` seen as `Custom Types`) for example'''
  // we might see `Trait` in the future or use it in our examples...
  // let;s keep this example not over complicating to focus on `Box`
  vect: &mut Vec<Box<dyn Any>>) {
    // we can also put the types that we want like `u64` for example
    // Rust default type for `Integer` is `i32` because more efficient
    // we can also write the `Integer` with its `Type` like that:
    // eg for i8: `Box::new(-2i8)` // like that!!!
    // no need `as Box<dyn Any>` type is already known
    vect.push(Box::new(-2));
    // Rust default type for `Float` is `f64` because more precised
    vect.push(Box::new(5.01));
    // Rust default type for `String` is `String`
    vect.push(Box::new(String::from(
     "Yoyogi Park can be accessed from Harajuku and Shibuya."
    )));

    // now let's use `match` or `if let` to check on those values stored in the `Vec`
    // by downcasting at a particular index point from the `Vec`
    if let Some(val) = vect[0].downcast_ref::<i32>() {
      println!("Found i32: {:?}", val);
    }
    if let Some(val) = vect[1].downcast_ref::<f64>() {
      println!("Found f64: {:?}", val);
    }
    if let Some(val) = vect[2].downcast_ref::<String>() {
      println!("Found String: {:?}", val);
    }

    println!("################################");
    // lets create an iterator to make it spicy and use `match`
    for (index, elem) in vect.iter().enumerate() {
      // don't forget that Box<dyn Any> returns an `Option` (Some(), None)
      // let use that to chain the possibilities
      match elem.downcast_ref::<i32>() {
        // val is a placeholder , you can put anything
        Some(val) => println!(
          "Index {:?}: i32 = {:?}", index, val),      
        None => match elem.downcast_ref::<f64>() { 
          Some(val) => println!(
            "Index {:?}: f64 = {:?}", index, val),
          None => match elem.downcast_ref::<String>() {
            Some(val) => println!(
              "Index {:?}: String = {:?}", index, val),
            None => println!(
              "Index {:?}: Unknown type = {:?}", index, elem
            ),
            },
          },
        }
    }

}
*/

// example 4: using an `enum` as `Box` Type 
// this one will be less heavy than the previous 
// example which was the big one to really understand
// but know that you can use `Enum` as `Type`
pub fn custom_enum_box_check(custom_box: Box<dyn Any>) {
  if let Some(our_custom_enum) = custom_box
    .downcast_ref::<MangaKissaZone>() {
      match our_custom_enum {
        MangaKissaZone::Attendance(placeholder) =>
          println!(
            "Attendance of Shibuya Manga Kissa is: {}",
            placeholder
          ),
        MangaKissaZone::Temperature(can_be_anything) =>
          println!(
            "Temperature of Shibuya Manga kissa is: {}",
            can_be_anything
          ),
        MangaKissaZone::Comment(secret_comment) =>
          println!(
            "Comment about Shibuya Manga Kissa is: {}",
            secret_comment
          ),
      }
  }
}







