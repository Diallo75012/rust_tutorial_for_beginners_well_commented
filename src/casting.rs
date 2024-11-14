// Example 1
pub fn normal_casting() {
  let total_manga_read: u32 = 42;
  let cost_per_manga: f64 = 5.5;
  // casting u32 to f64 to ensure proper multiplication
  let total_cost: f64 = total_manga_read as f64 * cost_per_manga;

  println!(
    "The total cost for reading mangas is: {:.2} JPY",
    total_cost
  );
}

// Example 2

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
    self.rating.into()
  }
}

pub fn generics_in_structs() {
  let rating: MangaRating<u32> = MangaRating { rating: 85 };
  println!(
    "Original Rating: {:?}",
    rating
  );
  println!(
    "Rating as Float: {}",
    rating.rating_as_float()
  );

}

// Example 3
// here we will be doing our complex example
// with and `enum` to represent multi variants
// with a generic struct
// with an `impl` that will use casting to handle scenarios

use std::convert::From;
use std::fmt::Display;
use std::ops::Add;

#[derive(Debug)]
enum MangaKissaType {
  Basic(u32),
  Deluxe(f64),
  Vip(String),
}

#[derive(Debug)]
struct MangaService<T> {
  cost: T,
  service_type: MangaKissaType,
}

impl<T> MangaService<T>
where
  // `T` as to implement `Add` to add up,
  // `From` to cast type to `T` types
  // Copy to not move ownership
  // Display for the printing using `{}`
  T: Add<Output = T> + From<u32> + Copy + Display,
{
  fn upgrade_service(&self, extra_cost: T) -> T {
    // as `T` implements `Add` we can use `+` operation
    self.cost + extra_cost
  }
}

// implementing `From` trait to convert `MangaKissaType` to f64
// `From` helps to be able to use it where ever we want to
// convert `MangaKissaType` to `f64` instead of having specific separate functions
impl From<MangaKissaType> for f64 {
  fn from(manga_type: MangaKissaType) -> f64 {
    match manga_type {
      // here cost is an `u32` so we cast it to an f64
      MangaKissaType::Basic(cost) => cost as f64,
      // here `cost` is already an f64
      MangaKissaType::Deluxe(cost) => cost,
      // Default value for Vip type
      MangaKissaType::Vip(_) => 1000.0,      
    }
  }
}

pub fn manga_kissa_services() {
  let deluxe_service = MangaService {
    cost: 150.5,
    service_type: MangaKissaType::Deluxe(150.5),
  };

  let upgraded_cost = deluxe_service.upgrade_service(50.0);
  println!(
    "Upgraded Cost: {}",
    upgraded_cost
  );

  // we can do this because we have implemented `From`
  // to convert `MangaKissaType` to `f64`
  // we could use `.into()`:
  // let converted_cost: f64 = deluxe_service.service_type.into();
  // Into trait is automatically implemented when From is implemented
  let converted_cost: f64 = f64::from(deluxe_service.service_type);
  println!(
    "Converted Cost from Enum: {}",
    converted_cost
  );
}
