// now that we have seen several concepts,
// you won't be affraid to see me putting some decorators
// we are going to allow dead_code and active debug for enums

// lets not forget to import `Any` trait to be able to use `any` type
// we do not need to import `Box` like we did in previous video for `HashMap` 
use std::any::Any;


#[allow(dead_code)]
#[derive(Debug)]
pub enum MangaKissaZone {
  Attendance(u64),
  Temperature(f64),
  Comment(String),
}

// example 1: print enum

// note here that this is trick to allow a `Vector`,
// which is normally accepting only one type,
// to hold different types using an `Enum` (`Vec<MangaKissaZone>`)
pub fn print_manga_kissa_zone_debug_mode(vec: Vec<MangaKissaZone>) {
  println!("Vector is: {:?}", vec)
}



// example 2: downcasting values from `Box`
pub fn box_simple_downcasting(vec_simple_box: Vec<Box<dyn Any>>) {

  for elem in vec_simple_box.iter() {

    // retrieve values by downcasting each element type
    // and converting those if needed to
    // this is possible because we have `Any` which helps to know type at `runtime`
    // but this means that it adds more checks at runtime and can alter performance
    // also it can't be used everywhere just for some specific cases it needed to perform operation
    // using the value behind the reference `Box`
    // using `Box<dyn Any>` it too much will increase the chances of getting runtime errors 
    // because of incorrect `Type` assumptions (Know What You Do!) 
    if let Some(val) = elem.downcast_ref::<i32>() {
      println!("Found i32: {}", val);
    } else if let Some(val) = elem.downcast_ref::<f64>() {
      println!("Found f64: {}", val);
    } else if let Some(val) = elem.downcast_ref::<String>() {
      println!("Found String: {}", val);
    } else {
      println!("Unknown type");
    }
  }
}


// example 3: Downcasting and mutating using reference `&mut` to make it more fun

// that is the type `Box<dyn Any>` used for boxes
// here whatever `Any` is inside `Vec` will be put on `Heap` memory
// while the `Box` reference stays on the Stack
// the `dyn` for `Dynamic` will permit the compiler to understand that `Type` is not known
// at compile time and Rust will have to figure out `Types` at `Runtime`
// learn here about the word `Polymorphism`:
// having a `Trait` interface for example being implemented by several different `Struct` (`Struct` seen as Custom types) for example 
// we might see `Trait` in the future or use it in our examples: let's keep this example not over complicated to focus on `Box`
pub fn box_downcasting_in_yoyogi(vect: &mut Vec<Box<dyn Any>>) {
  
  // `i` type of int: if we don't choose a type by default rust will use `i32`
  // because it is more efficient
  // otherwise we can tell the type like that:
  // `Box::new(-2<Put the type here>)`
  // eg. for -8: `Box::new(-2i8)` / like that!
  // all pushed boxes `Types` are inferred by Rust Automatically as `Box<dyn Any>`
  vect.push(Box::new(-2));
  // here type for Float is `f64` the default as it is more precise
  vect.push(Box::new(5.01));
  // `String`
  vect.push(Box::new(String::from("Yoyogi Park can be accessed from Harajuku and Shibuya.")));

  // now use `match` or `if let` to check on those values stored in the `Vec`
  // by downcasting at a particular index point from the `Vec`
  if let Some(val) = vect[0].downcast_ref::<&str>() {
    println!("Found &str: {:?}", val);
  }

  if let Some(val) = vect[1].downcast_ref::<i32>() { 
    println!("Found i32: {:?}", val);
  }

  if let Some(val) = vect[2].downcast_ref::<f64>() {
    println!("Found f64: {:?}", val);
  }

  // iterating through the vector
  // and printing elements  by downcasting types
  // when `downcasting` we need to `Match` the element type
  // in the vector if using exact index retrieval
  // therefore here can also use `Match` synthaxe or equivalent `if let`
  for (index, elem) in vect.iter().enumerate() {
    match elem.downcast_ref::<i32>() {
      Some(val) => println!("Index {:?}: i32 = {:?}", index, val),
      None => match elem.downcast_ref::<f64>() {
        Some(val) => println!("Index {:?}: f64 = {:?}", index, val),
        None => match elem.downcast_ref::<String>() {
          Some(val) => println!("Index {:?}: String = {:?}", index, val),
          None => println!("Index {:?}: Unknown type = {:?}", index, elem),
          },
        },
      } 
  }
  /*
  // `if let` syntaxe
  for (index, elem) in vec.iter().enumerate() {
    if let Some(val) = elem.downcast_ref::<i32>() {
      println!("Index {:?}: i32 = {:?}", index, val);
    } else if let Some(val) = elem.downcast_ref::<f64>() {
      println!("Index {:?}: f64 = {:?}", index, val);
    } else if let Some(val) = elem.downcast_ref::<String>() {
    println!("Index {:?}: String = {:?}", index, val);
    } else {
      println!("Index {:?}: Unkown type = {:?}", index, elem);
    }
  }
  */
}


// example 4: using an `enum` as `Box` Type
pub fn custom_enum_box_check(custom_box: Box<dyn Any>) {
  if let Some(our_custom_enum) = custom_box.downcast_ref::<MangaKissaZone>() {
     match our_custom_enum {
       MangaKissaZone::Attendance(val) => println!("Attendance of Shibuya Manga Kissa is: {}", val),
       MangaKissaZone::Temperature(val) => println!("Temperature of Shibuya Manga Kissa is: {}", val),
       MangaKissaZone::Comment(val) => println!("Comment about Shibuya Manga Kissa is: {}", val),
    }
  }
}
