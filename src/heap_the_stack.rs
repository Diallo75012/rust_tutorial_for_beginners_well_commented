// let create a fixed-size integer
// cargo add num-bigint/num-traits
use num_bigint::BigUint;
use num_traits::One;

pub fn move_your_heap() -> u128 {
  /*
  // example 1
  // u/i:8/16/64... are 'Stack' allocated 
  let memory_allocated = 42;
  let memory_allocated_clother = 109;
  // String: heap allocated
  let memory_allocated_different = String::from(
    "In Shibuya, You Move Your Heap!"
  );
  let memory_allocated_different_but_close = String::from(
    "Shinjuku is too busy, but Shibuya..."
  );

  // let's print memory addresses of 
  // our stack and heap vars
  // we are going to use `:p` to print the addresses
  // `{:p}` = pointer address
  // we use `&` to reference the variable so that we get there addresses
  println!(
    "Address stack of var 'memory_allocated': {:p}", 
     &memory_allocated
  ); // 0x7ffcb9846b58
  println!(
    "Address stack of var 'memory_allocated_clother': {:p}",
     &memory_allocated_clother
  ); // 0x7ffcb9846b5c
  println!(
    "Address stack of var 'memory_allocated_different': {:p}",
    &memory_allocated_different
  ); // 0x7ffcb9846b60
  println!(
    "Address stack of var 'memory_allocated_different_but_close': {:p}",
    &memory_allocated_different_but_close
  ); // 0x7ffcb9846b78

  // we can also print the pointer to its content
  // using the method `.as_ptr()`
  // the &str references of the `String` variables
  // contain some information...
  // a way to get the data which the pointer is referencing to. 
  println!(
    "Information contained in pointer reference of `String` '&memory_allocated_different': {:?} ",
    &memory_allocated_different
  );
  */


  // example2: StackOverFlow the website! no?
  // this tutorial have been copied from Stackoverflow
  // ok lets create one actually
  // lets see after how to prevent it to happen
  // we using here recursion and will use a `nbr` large enough
  // i create a variable above so that you can see that we can do that in Rust
  // for `u64` 20 OK, 21 Stackoverflow, for `u128` 34 OK, 35 Stackoverflow
  let shibuya_crossing_crowd = big_number_exploding_linux_stack(34);

  //#[allow(unconditional_recursion)]
  /*
  fn big_number_exploding_linux_stack(nbr: u64) -> u64 {
    if nbr == 1 {
      1
    } else {
        nbr * big_number_exploding_linux_stack(nbr - 1)
    }
  }
  */
  // now how to not `Stack Over Flow!` ? 
  // we need to use a `loop` as stated/suggested by the compiler
  fn big_number_exploding_linux_stack(nbr: u128) -> u128 {
    let mut total = 1;
    for n in 1..=nbr {
      total *= n;
    }
    total
  } 

  // for loop example with Crates num_bigint and num_traits (cargo add)
  // make sure to chang the function returned type to `BigUint`
  /*
  fn crowd_count(nbr: u64) -> BigUint {
    let mut total = BigUint::one(); 
    for n in 1..=nbr {
      total *= n;
    }
    total
  }
  */

  shibuya_crossing_crowd
}
