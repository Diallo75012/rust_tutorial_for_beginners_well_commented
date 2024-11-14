/*
// Example 1
pub fn normal_casting() {
  let total_manga_read: u32 = 42;
  let cost_per_manga: f64 = 5.5;

  // let's cast from u32 to f64 to ensure proper multiplication
  let total_cost: f64 = total_manga_read as f64 * cost_per_manga;

  println!(
    // `{:.2}` for how many digits we want after the comma
    "The total cost for reading mangas is: {:.2} JPY",
    total_cost
  );
}


// Example 2:

fn print_type_of<T>(_: &T) {
  println!(
    "{}",
    std::any::type_name::<T>()    
  );
}

#[derive(Debug)]
struct MangaRating<T> {
  rating: T,
}

impl<T> MangaRating<T>
where
  T: Into<f64> + Copy,
{
  fn rating_as_float(&self) -> f64 {
    // convert generic rating to f64
    // because we have the type `T` implementing `Into`
    // we can use `.into()`
    self.rating.into()
  }
}

pub fn generics_in_structs() {
  let rating: MangaRating<u32> = MangaRating { rating: 85 };
  println!(
    "Original Rating: {:?}",
    rating
  );
  print_type_of(&rating);
  println!(
    "Rating as Float: {}",
    rating.rating_as_float()
  );
  print_type_of(&rating.rating_as_float());
}
*/

// Example 3:
// here we will be doing our complex example
// with `enum` to represent multi variant
// with generics..
// with `impl` that will use casting to handle scenarios

use std::convert::From;
use std::fmt::Display;
use std::ops::Add;

// enum with different types of services
#[derive(Debug)]
#[allow(dead_code)]
enum MangaKissaTypeEnum {
  Basic(u32),
  Deluxe(f64),
  Vip(String),
}

// a struct with cost and service type
#[derive(Debug)]
struct MangaServiceStruct<T> {
  cost: T,
  service_type: MangaKissaTypeEnum,
}

impl<T> MangaServiceStruct<T>
where
  // `T` has to implement `Add` to "add up" (+ operations)
  // `From` to cast type to `T` types
  // `Copy` to not move ownership
  // `Display` for the printing using `{}`
  T: Add<Output = T> + From<u32> + Copy + Display,
{
  // let's make our function inplementation
  fn  upgrade_service(&self, extra_cost: T) -> T {
    // as `T` implements `Add` now, we can use `+` operation
    self.cost + extra_cost
  }
}

// let's implement a convertion for the enum type
// implementing `From` trait to convert `MangaKissaTypeEnum` to f64
// `From` helps to be able to use it where ever we want to
// convert `MangakissaTypeEnum` to `f64` instead of having
// specific sperate functions for each cases
impl From<MangaKissaTypeEnum> for f64 {
  fn from(manga_type: MangaKissaTypeEnum) -> f64 {
    // let's use match pattern for our possible occurences
    match manga_type {
      // here `cost` is an `u32` we use `as` to convert it
      MangaKissaTypeEnum::Basic(cost) => cost as f64,
      // here `cost` is already an f64: no conversion needed
      MangaKissaTypeEnum::Deluxe(cost) => cost,
      // here we just a default f64 value for Vip (ichiman yen)
      MangaKissaTypeEnum::Vip(_) => 10000.0,
    }
  }
}


pub fn manga_kissa_services() {
  let deluxe_service = MangaServiceStruct {
    cost: 7500.5,
    service_type: MangaKissaTypeEnum::Deluxe(7500.5),
  };

  let upgraded_cost = deluxe_service.upgrade_service(2499.5);
  println!(
    "Upgraded Cost: {}",
    upgraded_cost
  );


  // we can do this because we have implemented `From`
  // to convert `MangaKissaTypeEnum` to `f64`
  // we could use `.into()` like that:
  // let converted_cost: f64 = deluxe_service.service_type.into();
  // `Into` trait is automatically implemented when `From` is implemented
  let converted_cost: f64 = f64::from(
    deluxe_service.service_type
  );
  println!(
    "Converted Cost from Enum: {}",
    converted_cost
  );
}



