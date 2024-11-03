// let's do the initial imports
use std::thread;
#[allow(unused_imports)]
use std::time::Duration;
#[allow(unused_imports)]
use std::sync::mpsc;

// Example 1:
// we are going to create our first `pub` fn
/*
pub fn multithreading_mangakissa_parallel() {
  // `spawn()` here is a closure
  // therefore we use `|| {...}`
  let _spawned_thread = thread::spawn( || 
    {
      for num in 0..50 {
        println!(
          "I am having {} free drinks in the manga kissa.",
          num
        );
        // sleep used only in this example for simulation
        thread::sleep(Duration::from_millis(1));
      }
  });

  let _spawned_thread_2 = thread::spawn( ||
    {
      for num in 0..50 {
        println!(
          "I am reading {} free books in the manga kissa.",
          num
        );
        thread::sleep(Duration::from_millis(1));
      }
  });

  for num in 0..50 {
    println!(
      "I am hidding in cabin {} of the manga kissa.",
      num
    );
    thread::sleep(Duration::from_millis(2));
  }
}


// Example 2:
pub fn multithreading_mangakissa_waiting_thread_to_finish_first() {
  let spawned_thread = thread::spawn( ||
    {
      for num in 1..11 {
        println!(
          "I am having {} free drinks in the manga kissa.",
          num
        );
      }
  });

  // here we use `.join()` like in Python 
  // to wait for the thread to finish 
  // before executing the rest of the code
  // as we are not using `.unwrap()` the compiler
  // propose to use `let _ = ...join();`
  // but we will comment this out and use
  // `match` instead of `.unwrap()`
  //let _ = spawned_thread.join();

  // safely `.join()` the thread without `.unwarp()`
  // as it could 'panic' if error in thread
  // lets do the `match`
  match spawned_thread.join() {
    Ok(_) => println!(
      "Thread finished successfully!"
    ),
    Err(e) => eprintln!(
      "Error joining thread: {:?}",
      e
    ),
  }

  for num in 1..11 {
    println!(
      "Who is hidding in cabin {} of the mangakissa?",
      num
    );
  }
}


// Example 3:
// moving ownership of variable as the thread 
// runs in another scope {} parallel to the `fn`
// general scope
pub fn multithreading_mangakissa_moving_ownership_if_using_variable(
  v: Vec<u64>
) {
  // so here because we use an owned vector
  // as `fn` parameter,
  // we will move that ownership
  // we need an owned variable in the 
  // "Thread closure"
  // so you can also "Clone" (`v.clone()`)
  // what we are trying to avoid here is issues
  // with `Lifetimes` and borrowing issues
  // different scopes issues...
  // so OR you  use clear lifetimes (headacke)
  // OR you move ownership (best easier cool nice for me)
  // or you clone (still need to move ownership 
  // but if you don't need to reuse that 
  // variable somewhere else other than the 
  // thread it is fine!)
  // we are done with the long story!
  let spawned_thread = thread::spawn( move ||
    {
      // now `v` is owned in this scope
      for num in v.iter() {
        println!(
          "I am increading my listening chakra to x{} to understand Naruto while volume is down to zero to not disturb people in the manga kissa.",
          num
        );
      }
  });

  // let's match directly here
  match spawned_thread.join() {
    Ok(_) => println!(
      "Thread finished successfully!"
    ),
    Err(e) => eprintln!(
      "Error joining thread: {:?}",
      e
    ),
  }

  for num in 1..11 {
    println!(
      "I am watching Naruto in open space {} of the mangakissa without any sound.",
      num
    );
  }

}


// Example 4:
// Channels: threading but passing messages from
// threads to other threads
// special import for that
use std::sync::mpsc;

// we are going to use our function used in
// previous videos to print the "Types" for
// analysis purpose and to get informed about
// interesting variable types to analyse!
fn print_type_of<T>(_: &T) {
  println!(
    "{}",
    std::any::type_name::<T>()    
  );
}


pub fn thread_message_passing(first_name: &str) {
  // we are going to use `channel()` 
  // which returns a Tuple
  let (transmitter, receiver) = mpsc::channel();

  // we need to own the value 
  // and have the owned value moved in the thread 
  // so compiler won't cry!
  // because `fn` scope lifetime is smaller than the thread on
  // now you get it that function can end 
  // while the thread is still working or processing stuff 
  // which can causelifetime issues...
  // we repeat again but we are sure to understand like that!
  let name = String::from(
    // `&str` = string literal
    first_name
  );
  println!("NAME before move: {}", name);

  // we can `thread` directly as well 
  // without calling any variable (no `let var...`)
  thread::spawn( move || 
    {
    // OR use `static` lifetime reference like:
    // `first_name: &'static str` as `fn` paramater
    // and declare here the `name` variable
    // so that its lifetime is linked to the thread
    // socpe and not the `fn` one
    // `let name = String::from(first_name);`
    // those are the possible Types returned by
    // `.send()` from the `transmitter`
    // CORE Result: core::result::Result<(), 
    // MPSC: SendError<alloc::string::String>>
    let sent_name = transmitter.send(name);
    // this is an interesting vaeriable to check
    print_type_of(&sent_name);

    // instead of `match` let's vary 
    // and see `if let` way of pattern matching
    if let Err(e) = sent_name {
      println!(
        "Error joining thread: {:?}",
        e
      );
    } else {
      println!(
      "Thread message successfully sent: {:?}",
      sent_name
      );
    }

  });

  // we have sent, now let's receive
  // those are the types that can be returned by
  // `.recv()` from the `receiver`
  // CORE Result: Result<alloc::string::String, 
  // MPSC: RecvError
  // INTERESTING!!!!
  let received_name = receiver.recv();
  // also this is interesting variable to check
  print_type_of(&received_name);
  // here let's use the `match` way to pattern match
  // so that you see some variant of how it can be handled differently
  match received_name {
    // `name_of_the_person` is just a placeholder
    // put whatever you find explicative to you
    Ok(name_of_the_person) => println!(
      "{:?} left the mangakissa at 00:03:43",
      name_of_the_person
    ),
    Err(e) => println!(
      "Error joining thread: {:?}",
      e
    ),
  }


}
*/


// Example 5:
// using more cores to compute a result in a
// quicker way using "Multiple" transmitters
// and "One" receiver
// Always "one receiver" but producers can be "multiple"
// we use that here to make heavy calculations
// that the most powerful quantum computer on earth
// can't do but Rust can!
// Our Rule: "Multi Producers, Single Consumer" (mpsc)


pub fn multithread_parallel_produce_and_receive_once_to_calculate_result(
  v: &Vec<u64>
) {
    let (transmitter, receiver) = mpsc::channel();

    // spawn a thread for each element in the vector
    for i in 0..v.len() {
      // clone the transmitter for each thread
      let tx_clone = transmitter.clone();
      // get the current number from the vector
      let value = v[i];
  
      thread::spawn(
        move || {
          // simulate a calculation that no one can do on earth
          let result = value * 200710904;
          // send the result via channel 
          // and handle error 
          // using `if let` this time 
          // (just to vary , can use match)
          if let Err(e) = tx_clone.send(result) {
            println!(
              "Error sending result: {:?}",
              e
            );
          } else {
            println!(
             "Thread successfully sent result: {}",
             result
            );
          }
      });
   }

  // inital transmitter has never been used 
  // as we just cloned it in the for loop
  // therefore, we need to drop it here
  // otherwise `receiver` will be still waiting
  // to receive something...
  drop(transmitter);

  // prepare a variable to collect 
  // the sum of the results
  let mut total: u64 = 0;

  // receive results from the threads one by one 
  // and we push those one by one to a `result_vec`
  // we ensure to receive the correct number of 
  // results by using a for loop with `v.len()`
  for _ in 0..v.len() {
    // handle error using `match`
    match receiver.recv() {
      // collect the received data summing it up 
      // in `total`
      Ok(data) => total += data,
      // handle receiving errors
      Err(e) => println!(
        "Error receiving data: {:?}",
        e
      ),
    }
  }

  // print the total sum
  println!("Total: {}", total);

}








