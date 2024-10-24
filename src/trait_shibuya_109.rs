// we are going to create a method shared across all businesses
// and returns a service description
// let's now use `Generic` which will help prevent errors
// for misuse of type.
// Trait = "Shared Inteface"

use std::fmt;
use std::ops::Div;

trait Service<T> {
  // example 1: `signature` method
  fn provide_service(&self) -> String;
  // flexible data for this field
  fn get_paid(&self, cost: &T) -> String;
}

/*
// we are going now to create `struct` for different businesses
#[derive(Debug)]
struct Karaoke<T> {
  service: String,
  number_of_people: u64,
  cost: T,
}

#[derive(Debug)]
struct MangaKissa<T> {
  service: String,
  number_of_people: u64,
  cost: T,
}
 
#[derive(Debug)]
struct Store<T> {
  service: String,
  address: String,
  cost: T,
}
*/

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
  cost: T,
}


// now let's implement the specific logic from the `signature` Trait method
/*
impl Service<u64> for Karaoke<u64> {
  fn provide_service(&self) -> String {
    let private_room_service = format!(
      "{:?} is available for private Karaoke rooms!",
      self.service
    );
    private_room_service
  }

  fn get_paid(&self, cost: &u64) -> String {
    let total_cost_per_person = cost / self.number_of_people;
    let total = format!(
      "Total Cost Per Person Will be: {:?} Yens",
      total_cost_per_person
    );
    total
  }
 
}
*/

/*
// `Service` implementation fully Generic 
// for the field `cost` on `Karaoke` struct
impl<T> Service<T> for Karaoke<T>
where
  T: fmt::Display + Copy + Div<u64, Output = T>, {
  fn provide_service(&self) -> String {
    format!(
      "{} is available for private Karaoke rooms!", 
      self.service
    )
  }

  fn get_paid(&self, cost: &T) -> String {
    let total_cost_per_person = *cost / self.number_of_people;
    format!(
      "Total Cost Per Person Will be: {} Yens", 
      total_cost_per_person
    )
  }
}
*/

// now we make it fully flexible using Generic in an efficient way
// here for two fields from the struct helped to make 
// implementation very powerful
// Implement the Service trait for Karaoke, using generic T and N
impl<T, N> Service<T> for Karaoke<T, N>
where
  T: fmt::Display + Copy + Div<N, Output = T>,
  N: Copy, {

  fn provide_service(&self) -> String {
    format!(
      "{} is available for private Karaoke rooms!", 
      self.service
    )
  }

  fn get_paid(&self, cost: &T) -> String {
    let total_cost_per_person = *cost / self.number_of_people;
    format!(
      "Total Cost Per Person Will be: {} Yens", 
      total_cost_per_person
    )
  }
}

/*
impl Service<f64> for MangaKissa<f64> { 
  fn provide_service(&self) -> String { 
    let vip_client_service = format!(
      "{:?} is available for VIP clients for free!",
      self.service
    );
    vip_client_service

  }

  fn get_paid(&self, cost: &f64) -> String {
    let total_cost_per_person = cost / self.number_of_people as f64;
    let total = format!(
      "Total Cost Per Person Will be: {:?} Yens",
      total_cost_per_person
    );
    total
  }

}
*/

/*
// `Service` implementation fully Generic 
// for the field `cost` on `MangaKissa` struct
impl<T> Service<T> for MangaKissa<T>
where
  T: fmt::Display + Copy + Div<f64, Output = T>, {
  fn provide_service(&self) -> String {
      format!(
        "{} is available for VIP clients for free!", 
        self.service
      )
  }

  fn get_paid(&self, cost: &T) -> String {
    let total_cost_per_person = *cost / self.number_of_people as f64;
    format!(
      "Total Cost Per Person Will be: {} Yens", 
       total_cost_per_person
    )
  }
}
*/

// Implement the Service trait for MangaKissa, using generic T and N
impl<T, N> Service<T> for MangaKissa<T, N>
where
  //T: fmt::Display + Copy + Div<N, Output = T>,
  //N: Copy, 
  T: fmt::Display + Copy + Div<T, Output = T> + From<f64>,
  N: Copy + Into<u64>, {
  fn provide_service(&self) -> String {
    format!(
      "{} is available for VIP clients for free!", 
      self.service
    )
  }

  fn get_paid(&self, cost: &T) -> String {
    // Convert `number_of_people` `N` to `u64` first, 
    // then convert to `T` to be able to divide
    let nbr_of_people_as_t: T = T::from(
      self.number_of_people.into() as f64
    );

    let total_cost_per_person = *cost / nbr_of_people_as_t;
    format!(
      "Total Cost Per Person Will be: {} Yens", 
      total_cost_per_person
     )
  }
}


/*
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
      cost, self.address
    );
    total
  }
}
*/


// `Service` implementation fully Generic 
// for the field `cost` on `Store` struct
impl<T> Service<T> for Store<T>
where
  T: fmt::Display + Copy + Div<u32, Output = T>, {
  fn provide_service(&self) -> String {
    format!(
      "{} is available for people spending more than 5000 Yen!",
      self.service
    )
  }

  fn get_paid(&self, cost: &T) -> String {
    format!(
      "Total Cost is {} Yens and will be delivered to {}.",
      cost, self.address
    )
  }
}




pub fn trait_required_for_109_shibuya_tower() {
  // instantiate variables for each `struct`
  let karaoke_109 = Karaoke {
    service: String::from("Golden Mic Dolby Surround Galactic"),
    number_of_people: 7u64,
    cost: 12300u64,
  };
  println!("Karaoke 109: {:?}", karaoke_109);
  
  let manga_kissa_109 = MangaKissa { 
    service: String::from("Udon and Tempura Food"),
    number_of_people: 4u64,
    cost: 8973.0f64,
  };
  println!("Manga 109: {:?}", manga_kissa_109);


  let junko_store_109 = Store { 
    service: String::from("Home Delivery Service"),
    address: String::from("3-27 Komazawa Daigaku, Setagaya, Tokyo"),
    cost: 7900u32,
  };
  println!("Junko Store 109: {:?}", junko_store_109);



  // use `Trait` method
  println!(
    "{}", karaoke_109.provide_service()
  );
  println!(
    "{}", karaoke_109.get_paid(&karaoke_109.cost)
  );
  println!(
    "{}", manga_kissa_109.provide_service()
  );
  println!(
    "{}", manga_kissa_109.get_paid(&manga_kissa_109.cost)
  );
  println!(
    "{}", junko_store_109.provide_service()
  );
  println!(
    "{}", junko_store_109.get_paid(&junko_store_109.cost)
  )

}




