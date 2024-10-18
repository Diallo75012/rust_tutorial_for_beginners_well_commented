// let create a fixed-size integer
use num_bigint::BigUint;
use num_traits::One;


pub fn move_your_heap() -> BigUint {
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

  // example 2: StackOverFlow the website? 
  // this tutorial have benn copied from Stackoverflow.com ;)
  // Ok let's create one actually
  // let's see after how to prevent it to happen
  // we are going to use recursion and use a `nbr` to quickly get big values
  // so here for `nbr=20` we got: `2432902008176640000`
  // very long number but if we exceed the capacity of the stack `u64` allocated type...
  // we try a much larger `nbr`
  // but same here there is a limit
  let shibuya_crossing_crowd = big_number_exploding_linux_stack(1_000);

  //#[allow(unconditional_recursion)]
  // so with that no need anymore to use a decorator to allow recursion
  // but let's use the loop way to not Stackoverflow
  // as advised by the our firend compiler ;)
  /*
  fn big_number_exploding_linux_stack(nbr: u64) -> u64 {
    if nbr == 1{
      1
    } else {
        nbr * big_number_exploding_linux_stack(nbr -1)
    }
  } 
  */

  // here the loop way
  // we keep this function but does it mean that we are OK?
  // we will increase the number....
  // now things become interesting as we could just say
  // OK then I am going just to increase that `u` type..
  // but in std Rust there is a limit `u128`
  // so let's go step by step to understand that and use `u128`
  /*
  fn big_number_exploding_linux_stack(nbr: u128) -> u128 {
    let mut total = 1;
    for n in 1..=nbr {
      total *= n;
    }
    total
  }
  */

  // therefore this is a solution but you have seen the limits of it
  // we will need to import packages `Crates` that will permit us to do that and not explode the stack
  // probably those packages are using stuff like `Box<T>` in order to have those variables on the `Heap`

  fn big_number_exploding_linux_stack(nbr: u64) -> BigUint {
    let mut total = BigUint::one();
    for n in 1..=nbr {
      total *= n;
    }
    total
  }

  shibuya_crossing_crowd
}







