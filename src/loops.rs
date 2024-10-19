// we have seen hashmaps/vectors/arrays
// we are going to use vectors and maybe hashmaps
// to focus on different ways to loop over those
// HashMap need to be imported as we have seen in the explanation part
#[allow(unused_imports)]
use std::collections::HashMap;


pub fn loop_ways() -> Vec<u64> {
  // this will be our returned `Vec`
  // let mut super_vector: Vec<u64> = Vec::new();

  /*
  // first way: `.iter()` (for Vector/HashMap/Array)
  let people_count_in_komazawa: Vec<u64> = vec![
    98, 23, 67, 124, 31, 18, 97, 44, 306, 109
  ];
  println!("people_count_in_komazawa: {:?}", people_count_in_komazawa);

  // let's iterate through and then comment a bit
  // we could have used a more direct way of iterating:
  // doing: `for people in &people_count_in_komazawa {...}`
  // Here `iter()` is using the `&` reference value under the hood
  for people_count in people_count_in_komazawa.iter() {
    // it is not done yet... `people_count` is actually an `&people_count`
    // we can evalute it vs another `&` therefore another ``<&u64>
    // or use `*` to evaluate it and compare to a <u64> 
    // this ways people_count<&u64> is showing the value behind the pointer 
    // we could have use instead: `if people_count > &50`
    if *people_count > 50 {
      // this is playing with the borrowing/owning/referencing rules
      // do it what best suits you...
      // that is why here we use `*` to dereference the pointer
      // and get the value <u64> that it is pointing to
      // we pass from `&people_count` to `people_count` using `*`
      super_vector.push(*people_count)
    }
  };

  // we return the vector result
  super_vector
  */

  /*
  // second way: `.iter_mut()` (for Vector/HashMap)
  // we define an `HashMap` (which equivalent of Python `dict`)
  // `[(K,V),...]`
  let mut people_count_in_setagaya: HashMap<String, u64> = HashMap::from(
    [
      ("Monday".to_string(), 39),
      ("Tuesday".to_string(), 54),
      ("Wednesday".to_string(), 109),
      ("Thursday".to_string(), 3),
      ("Friday".to_string(), 1290),
      ("Saturday".to_string(), 5949),
      ("Sunday".to_string(), 75),
  ]);
  println!("people_count_in_setagaya {:?}: ", people_count_in_setagaya);
 
  // here `day` and `count` will be references `&`
  // therefore `&day` for keys and `&count` for values
  for (day, count) in people_count_in_setagaya.iter_mut() {
    if day.contains("r") {
      // we use again `*` to get the value
      // = deference the pointer `&count` to get `count`
      super_vector.push(*count)
    }
  }

  super_vector
  */  


  /*
  // third way: `into_iter` (for Vector/Array)
  // let's declare an `Array` so that you see all types
  // 8 best restaurant's capacity
  let shibuya_8_udon_restaurant_capacity: [u64;8] = [
    237, 324, 432, 56, 12, 7, 76, 49
  ];
  println!("shibuya_8_udon_restaurant_capacity {:?}: ", shibuya_8_udon_restaurant_capacity);


  // here `into_iter()` is different from the other iterators
  // in terms of borrowing/owning/referencing life!!!!
  // `into_iter()` moves the ownership, therefore, no `&` here!
  for capacity in shibuya_8_udon_restaurant_capacity.into_iter() {
    // here `capacity` is owned not an `&capacity`
    if capacity % 2 == 0 {
      // therefore here we don't need `*` as no pointer
      // we working 100% with owned variable moved from here to there ;)
      super_vector.push(capacity)
    }
  }

  super_vector
  */

  /*
  // fourth way: looping in a function way (The Fun one!)
  // using methods available
  let mut japan_travel_rating: HashMap<String, u64> = HashMap::from(
    [
      // 100 is top ranking, o is the worst .... ;)
      ("2005".to_string(), 120),
      ("2007".to_string(), 0),
      ("2008".to_string(), 50),
      ("2014".to_string(), 110),
      ("2024".to_string(), 300),
    ]
  );
  println!("japan_travel_rating {:?}: ", japan_travel_rating);
  
  // we ready for this one and careful about the details which makes it fun
  let worse_big_troublesome_years: HashMap<String, u64> = japan_travel_rating
    // here we won't need `*` because:
    // `.iter()` uses `&` of the values. 
    // Keys can't be changes only values are `mut`
    .iter()
    // then `.filter()` does the same by also `referencing` to references created by `.iter()`
    // therefore here we have a bdouble referencing `(&&V, &&K)`
    // but when using `closures` (we will see that in the future)
    // so when using `closures` Rust automatically dereference
    // one layer. we get then: `(&K, &V)` at `.filter()` level
    // this is like when you use `lambda` functions in Python
    .filter(|(_year, &rate)| rate < 51)
    // if it was a `Vector` we would have used `.cloned()` instead at this level
    // which would have dereferenced one layer
    // trying to `.clone()` HashMap `(&String, &u64)` tuple would need some transformation to values... just don't do it like that! ;)
    // but for `HashMap` we need to `.map()` it to be able to `.clone()`
    // as at the exit of `.filter()` we have `(&K, &V)`
    // converting `(&K, &V)` to `(K, V)`
    // no need `*`
    .map(|(year, &rate)| (year.clone(), rate))
    .collect();

    // let's now get rid of those troublesome year in Japan
    // we get all keys with `.keys()` and want to make a `Vector`	
    // here the type will be `Vec<&String>` so that we can loop
    // over those references directly 
    // using the variable name without `&`
    // if using `.map(|year| year.clone())` instead of `.clone()`
    // the type will be `Vec<String>` = 	a `Vec<Owned String>` 
    // at this time we would need to implement iteration over `&`	
    let troublesome_years = worse_big_troublesome_years
      .keys() // `&`
      .clone()
      // here `_` in the `Vec<_>` will let Rust infer the type
      // type will be `Vec<&String>`
      .collect::<Vec<_>>();


  // now we can loop over and clean up those bad years 
  // out from our HashMap
  for year in troublesome_years {
    japan_travel_rating.remove(year);  
  }

  println!("worse_big_troublesome_years {:?}: ", worse_big_troublesome_years);
  // HashMap<String, u64> so we change the function returned type
  japan_travel_rating
  */


  // last way: `.copied`
  // i am tired because of the previous example ;)
  // therefore this one is gonna be lighter

  let tokyo_sky_tree_visitors_count = Vec::from(
    [21, 23, 87, 43,98,45,109, 1200, 30]
  );
  println!("tokyo_sky_tree_visitors_count {:?}: ", tokyo_sky_tree_visitors_count);

  let even_visitor_count = tokyo_sky_tree_visitors_count
    // get `&` of values
    .iter() 
    // filter even numbers
    .filter(|&count| count % 2 == 0)
    // dereference and copies
    // could be `.clone()` as a told you in previous example which is OK for `Vector`	
    .copied()
    // collects into a `Vec<u64>`
    // we use this notation before `Vec<_>` to let Rust infer type
    // but you can also precise it
    .collect::<Vec<u64>>();

  // we need to change function return type to `<Vec<u64>>`
  even_visitor_count

}







