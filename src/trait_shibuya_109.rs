// we are going to create a method shared across all businesses
// and returns a service description
// our code structure
/*
    trait
      |
    struct
      | 
    implementation (normal method and signature one)
      |
    instantiation
      |
    use signature function implemented on struct
      |
    Terminal output
*/
// let's now use `Generic` Type which will help prevent errors

// need to import those x2 for our implementations
use std::fmt::Display;
use std::ops::Div;

trait Service<T> {
  // let create the signature method
  fn provide_service(&self) -> String;
  // let's create a function using flexible field
  fn get_paid(&self, cost: &T) -> String;
}

// we are going now to create `struct` for different businesses
// we have added fields and one `Generic` field
// but we need to have `<T>` Type defined 
// at the `struct` level as well

// now lets push it to super powers of
// Trait + Generic Types

#[derive(Debug)]
struct Karaoke<T, N> {
  service: String,
  number_of_people: N,
  cost: T,
}

#[derive(Debug)]
struct MangaKissa<T, N> { 
  service: String,
  number_of_people: N,
  cost: T,
}

#[derive(Debug)]
struct Store<T> { 
  service: String,
  address: String,
  cost: T
}

/*
// now let's implement the specific logic from
// the `signature` Trait method
impl Service for Karaoke {
  fn provide_service(&self) -> String {
    let provide_room_service = format!(
      "{:?} is available for private Karaoke rooms!",
      self.service
    );
    provide_room_service
  }
}

impl Service for MangaKissa { 
  fn provide_service(&self) -> String { 
    let vip_client_service = format!(
      "{:?} is available for VIP clients for free!",
      self.service
    );
    vip_client_service
  }
}

impl Service for Store { 
  fn provide_service(&self) -> String { 
    let extra_service = format!(
      "{:?} is available for people spending more than 5000 Yen!",
      self.service
    );
    extra_service
  }
}
*/

/*
// implementations with `Generic`
// with types chosen and written by us
// which is little bit more strict but more
// flexible than the example before this one
impl Service<u64> for Karaoke<u64> {
  fn provide_service(&self) -> String {
    let provide_room_service = format!(
      "{:?} is available for private Karaoke rooms!",
      self.service
    );
    provide_room_service
  }

  fn get_paid(&self, cost: &u64) -> String {
    let total_cost_per_person = cost / self.number_of_people;
    let total = format!(
      "Total Cost Per Person Will be: {:?} Yens.",
      total_cost_per_person
    );
    total
  }
}

impl Service<f64> for MangaKissa<f64> { 
  fn provide_service(&self) -> String { 
    let vip_client_service = format!(
      "{:?} is available for VIP clients for free!",
      self.service
    );
    vip_client_service
  }

  fn get_paid(&self, cost: &f64) -> String {
    // here cost is an `f64`
    // but number_of_people is `u64`
    // so we cast/convert it using `as`
    let total_cost_per_person = cost / self.number_of_people as f64;
    let total = format!(
      "Total Cost Per Person Will be: {:?} Yens.",
      total_cost_per_person
    );
    total
  } 

}

impl Service<u32> for Store<u32> { 
  fn provide_service(&self) -> String { 
    let extra_service = format!(
      "{:?} is available for people spending more than 5000 Yen!",
      self.service
    );
    extra_service
  }

  fn get_paid(&self, cost: &u32) -> String {
    let total = format!(
      "Total Cost is {:?} Yens and will be delivered to {}.",
      cost,
      self.address
    );
    total
  } 
}


// now we are going to implement "Full Generic"
// for the field `cost` of each Struct to be whatever type
// instead of <u64> or <u32> or even <f64>
// here we will need also
// to mark the generic type to `impl`: `impl<T>`
impl<T> Service<T> for Karaoke<T> 
where
  // we implement display so no need `:?` anymore
  // also to be able to make operations +,*,-, /
  // with Generic Types  we need `Div`
  // here Div will convert it to `u64`
  // and output after calculation the type `T` result
  // we need to import `Display & Div`
  T: Display + Copy + Div<u64, Output = T>, {
  fn provide_service(&self) -> String {
    let provide_room_service = format!(
      "{} is available for private Karaoke rooms!",
      self.service
    );
    provide_room_service
  }

  fn get_paid(&self, cost: &T) -> String {
    // For Generic type we will need to get the
    // the value using `*` from the reference `&T`
    // `number_of_people` is u64 as well, 
    // therefore we can perform operation
    let total_cost_per_person = *cost / self.number_of_people;
    let total = format!(
      "Total Cost Per Person Will be: {} Yens.",
      total_cost_per_person
    );
    total
  }
}

impl<T> Service<T> for MangaKissa<T> 
where
  T: Display + Copy + Div<f64, Output = T>, { 
  fn provide_service(&self) -> String { 
    let vip_client_service = format!(
      "{} is available for VIP clients for free!",
      self.service
    );
    vip_client_service
  }

  fn get_paid(&self, cost: &T) -> String {
    // here `*cost` will be an `f64`
    // and `number_of_people` is casted/converted 
    // to `f64` using `as`, therefore, operation can be performed
    let total_cost_per_person = *cost / self.number_of_people as f64;
    let total = format!(
      "Total Cost Per Person Will be: {} Yens.",
      total_cost_per_person
    );
    total
  } 

}
*/

// we have just `T` in the `Trait` definition
// while we have `<T, N>` both in the `Karaoke` struct
// therefore we indicate both for the `impl`
impl<T, N> Service<T> for Karaoke<T, N> 
where
  // so we use to convert/cast to `number_of_people`
  // type, here it has a "generic" `N` type
  // therefore we just replace it here with its "generic" type
  T: Display + Copy + Div<N, Output = T>,
  // so now you understand why we need `Copy`
  // as the compiler told us to `Clone` `N` for eg.
  N: Copy {
  fn provide_service(&self) -> String {
    let provide_room_service = format!(
      "{} is available for private Karaoke rooms!",
      self.service
    );
    provide_room_service
  }

  fn get_paid(&self, cost: &T) -> String {
    // here nothing changes
    // `*cost` will be casted to `N` type
    // just for the operation vs another `N` type (number_of_people)
    // then Output will be `T`
    // `total_cost_per_person` will be of type `T`
    // which already up there `impl` `Display`
    let total_cost_per_person = *cost / self.number_of_people;
    let total = format!(
      "Total Cost Per Person Will be: {} Yens.",
      total_cost_per_person
    );
    total
  }
}


// we do the same for MangaKissa~!!
impl<T, N> Service<T> for MangaKissa<T, N> 
where
  // but here because it is an f64 it is tricky
  // for T: we will have a `f64` so we 
  // casting the `T` during the Operation
  // generic types operation +,-,*./ need more work
  T: Display 
       + Copy 
       + Div<T, Output = T> 
       + From<f64>, 
  // so here the `N` will be a `u64` 
  // that we will be able to convert/cast 
  // during the operation to an `f64` like that
  // i will change this after to f64 so that you can see...
  // therefore we understand that the operation
  //  can be done on generics only if only those
  // are of the same type `T` here for example
  // that is why we convert it to `T` (cast `N` for `T`)
  //N: Copy + Into<f64> {
  N: Copy + Into<u64> { 
  fn provide_service(&self) -> String { 
    let vip_client_service = format!(
      "{} is available for VIP clients for free!",
      self.service
    );
    vip_client_service
  }

  fn get_paid(&self, cost: &T) -> String {
    // so to recap: `number_of_people` `N` Type
    // converted to `u64` first
    // then convert to `T` to be able to divide
    let nbr_of_people_as_t: T = T::from(
      //`into()` like in iterator moves ownership
      self.number_of_people.into() as f64
    );
    let total_cost_per_person = *cost / nbr_of_people_as_t;
    let total = format!(
      "Total Cost Per Person Will be: {} Yens.",
      total_cost_per_person
    );
    total
  } 

}

// we are not gonna change Store implementation
impl<T> Service<T> for Store<T> 
where
  T: Display + Copy + Div<u32, Output = T>, { 
  fn provide_service(&self) -> String { 
    let extra_service = format!(
      "{} is available for people spending more than 5000 Yen!",
      self.service
    );
    extra_service
  }

  fn get_paid(&self, cost: &T) -> String {
    let total = format!(
      "Total Cost is {} Yens and will be delivered to {}.",
      cost,
      self.address
    );
    total
  } 
}






pub fn trait_required_for_109_shibuya_tower() {
 
  // let's instantiate variables for each `struct`
  let karaoke_109 = Karaoke {
    service: String::from(
      "Golden Mic Dolby Surround Galactic"
    ),
    number_of_people: 7u64,
    cost: 12300u64,
  };
  println!("Karaoke 109: {:?}", karaoke_109);

  let manga_kissa_109 = MangaKissa { 
    service: String::from(
      "Udon and Tempura Food"
    ),
    number_of_people: 4u64,
    cost: 8973.0f64,
  };
  println!("Manga Kissa 109: {:?}", manga_kissa_109);

  let junko_store_109 = Store { 
    service: String::from(
      "Home Delivery Service"
    ),
    address: String::from(
      "3-27 Komazawa Daigaku, Setagaya, Tokyo"
    ),
    cost: 7900u32,
  };
  println!("Junko store 109: {:?}", junko_store_109);

  // we use methods implementing `Trait` `Service` signature
  println!(
    "{}",
    karaoke_109.provide_service()
  );
  println!(
    "{}",
    karaoke_109.get_paid(&karaoke_109.cost)
  );

  println!(
    "{}",
    manga_kissa_109.provide_service()
  );
  println!(
    "{}",
    manga_kissa_109.get_paid(&manga_kissa_109.cost)
  );

  println!(
    "{}",
    junko_store_109.provide_service()
  );
  println!(
    "{}",
    junko_store_109.get_paid(&junko_store_109.cost)
  )

}
















