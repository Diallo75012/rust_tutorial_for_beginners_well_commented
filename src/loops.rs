// we have seen hashmaps/vectors/arrays
// we are going to use vectors and maybe hashmap
// to focus on different way to loop over those
#[allow(unused_imports)]
use std::collections::HashMap;


pub fn loop_ways() -> Vec<u64> {
  // this will be the returned `Vec`
  //let mut super_vector: Vec<u64> = Vec::new();

  /*
  // first way: .iter() (for Vector/HashMap/Array)
  let people_count_in_komazawa: Vec<u64> = vec![
    98, 23, 67, 124, 31, 18, 97, 44, 306
  ];


  // we could have used instead:
  // for people_count in &people_count_in_komazawa {...}
  // more direct
  // here `.iter()` is using the `&` reference of the value
  for people_count in people_count_in_komazawa.iter() {
    // so from here `people_count` is a `&people_count`
    // we can evalute it vs another `&` therefore `&u64`
    // or use `*` to evaluate it and compare to a u64
    // other way is: if people_count > &50
    if *people_count > 50 {
      // that is why here we use `*` to dereference the pointer
      // and get the value that it is pointing to
      // we pass from `&people_count` to `people_count` using `*`
      super_vector.push(*people_count)
    }
  }
  
  super_vector
  */

  /*
  // second way: `.iter_mut()` (for Vector/HashMap)
  // we define out `HashMap` (like Python `dict`)
  // ... `[(K, V), ...]`
  let mut people_count_in_setagaya: HashMap<String, u64> = HashMap::from(
    [("Monday".to_string(), 39),
      ("Tuesday".to_string(), 54),
      ("Wednesday".to_string(), 109),
      ("Thursday".to_string(), 3),
      ("Friday".to_string(), 1290),
      ("Saturday".to_string(), 5049),
      ("Sunday".to_string(), 75),
  ]);

  // here `day` and `count` will be reference
  // therefore `&day` for keys and `&count` for values
  for (day, count) in people_count_in_setagaya.iter_mut() {
    // we use the available method `.contains` to check the keys
    if day.contains("r") {
      // we use again `*` to get the value
      // = dereference the pointer `&count` to get `count`
      super_vector.push(*count)
    }
  }

  super_vector
  */

  /*
  // thrid_way: into_iter (for Vector/Array)

  // let's declare an array of 8 best restaurant's capacity
  let shibuya_8_udon_restaurant_capacity: [u64;8] = [237, 324, 432, 56, 12, 7, 76, 49]; 

  // here `into_iter` is different from the others in terms of Ownership/Borrowing life!!!
  // `into_iter` moves the ownership, no `&` here!
  for capacity in shibuya_8_udon_restaurant_capacity.into_iter() {
    // here `capacity` is owned not an `&capacity`
    if capacity % 2 == 0 {
      // thereofre here we don't need `*`
      // we working 100% with owned variable moved from here to there
      super_vector.push(capacity)
    }
  }

  super_vector
  */


  /*
  // fourth way: looping in a functional way
  // using methods avaialable

  let mut japan_travel_rating: HashMap<String, u64> = HashMap::from(
    [
      // 100 is top ranking, 0 is the worst
      ("2005".to_string(), 120),
      ("2007".to_string(), 0),
      ("2008-2010".to_string(), 50),
      ("2014".to_string(), 110),
      ("2024".to_string(), 300),
    ]
  );
  println!("japan_travel_rating: {:?}", japan_travel_rating);


  let worse_big_troublesome_years: HashMap<String, u64> = japan_travel_rating.iter()
    // here we don't need * because:
    // `.iter()` uses `&` of the value
    // then `.filter()` does the same by also `reference` the reference created by `.iter()`
    // therefore here we have double referencing `(&&V,&&K)`
    // but when using `closures` Rust automatically dereference on layer so we have `(&V,&K)` instead of `(&&V,&&K)`
    // `.cloned()` used for `Vectors` in this case will then is going to dereference once layer
    // `.map()` used for `HashMap` will dereference the last layer `&` as well
    // that is why we use the `*` to be able evaluate the value and dereference the pointer to check `<51`
    .filter(|(_year, &rate)| rate < 51)
    // as we have said `cloned()` is going to dereference, to get values this for `Vector` but not for `HashMaps`
    //  PROBLEM for `HashMap` here is : 
    // Trying to clone a `(&String, &u64)` tuple directly which needs transformation to values directly... just don't do it like that!
    //.cloned()
    // Manually convert `(&K, &V)` to `(K, V)`
    .map(|(year, &rate)| (year.clone(), rate))
    // `.collect()` is going to rearrange the `(K,V)` back to a HashMap<String, u64>
    .collect();

  // lets now get rid of those troublesome years in Japan
  // be get alll keys and make a `Vector` out of those at the same time
  // here type is Vec<&String> so that we can loop over those reference directly using the variable name without `&`
  // if using `.map(|year| year.clone())` instead of `clone()`
  // the type will be Vec<String> = Vec<Owned String> at this time implement iteration over `&` references
  // here `_` in the Vec<_> will let Rust infer the type
  let troublesome_years = worse_big_troublesome_years.keys().clone().collect::<Vec<_>>();

  for year in troublesome_years {
    japan_travel_rating.remove(year);
  };

  println!("worse_big_troublesome_years: {:?}", worse_big_troublesome_years);
  japan_travel_rating
  */


  /*
  // last way: `.copied()`
  // here `copied()` used after `.iter()` will get the copies of the values instead of their references
  let tokyo_sky_tree_visitors_count = Vec::from([23, 87, 43, 98, 45, 109]);
  let even_visitor_count  = tokyo_sky_tree_visitors_count.iter()
    // filters even numbers
    .filter(|&count| count % 2 == 0)
    // dereferences and copies
    .copied()
    // collects into Vec<u64>
    .collect::<Vec<u64>>();

  even_visitor_count
 */

} 








