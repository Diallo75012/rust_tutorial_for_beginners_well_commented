// public function
pub fn range_fibo(val: u64) {
  // for loop `0` inclusive and `val` exclusive
  // for loop with `val` inclusive as well: `for num in 0..=val`
  for num in 0..val {
    println!("Fibo of {} is equal to {}", num, fibonacci(num));
  }
}

// private function
fn fibonacci(n: u64) -> u64 {
  // rust implicit returns
  if n == 0 {
    0
  } else if n == 1 {
    1
  } else {
    // we go recursively from here
    fibonacci(n - 1) + fibonacci(n - 2)
  }
}
