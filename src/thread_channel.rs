// let's do the initial imports
use std::thread;
#[allow(unused_imports)]
use std::time::Duration;


// Example 1:
// we are going to create our first `pub` fn
/*
pub fn multithreading_mangakissa_parallel() {
  // `spawn()` here is a closure therefore...
  let _spawned_thread = thread::spawn( || {
    for num in 0..50 {
      println!("I am having {} free drinks in the manga kissa.", num);
      thread::sleep(Duration::from_millis(1));
    }
  });

  let _spawned_thread_2 = thread::spawn( || {
    for num in 0..50 {
      println!("I am reading {} free books in the manga kissa.", num);
      thread::sleep(Duration::from_millis(1));
    }
  });
  
  for num in 0..50 {
    println!("I am hidding in cabin {} of the mangakissa.", num);
    thread::sleep(Duration::from_millis(2));
  }

}

// Example 2:
pub fn multithreading_managakissa_waiting_thread_to_finish_first() {
  // `spawn()` returns a `Result`
  let spawned_thread = thread::spawn( || {
    for num in 0..10 {
      println!(
        "I am having {} free drinks in the manga kissa.",
        num
      );
    }
  });

  // here we use `.join()` like in Python to wait for the thread to finish before executing the rest of the code
  // as we are not using `.unwrap()` the compiler proposed // to use `let _=` instead but we can handle with `match`
  // 1
  //let _ = spawned_thread.join();

  // OR safely join the thread without using `.unwrap()` as it could `panic`
  // if error in thread
  // safely handle join result with match
  // 2
  match spawned_thread.join() {
    Ok(_) => println!("Thread finished successfully!"),
    Err(e) => eprintln!("Error joining thread: {:?}", e),
  }

  for num in 0..10 {
    println!(
      "Who is hidding in can {} of the managakissa.",
      num
    );
  }
}



// Example 3:
// moving ownership of variable as the thread runs in another scope {} parallel to the function
// general scope
pub fn multithreading_mangakissa_moving_ownership_if_using_variable(v: Vec<u64>) {
  let spawned_thread = thread::spawn( move || {
    for num in v.iter() {
      println!(
        "I am increasing my listening chakra to x{} to understand Naruto while volume is down to zero to not disturb people in the manga kissa.",
        num
      );
    }
  });

  // here we use `.join()` like in Python to wait for the thread to finish before eecuting the
  // rest of the code
  // as we are not using `.unwrap()`
  // the compiler proposed to use `let _=` instead but we can handle it with `match` later on
  // let _ = spawned_thread.join();

  // OR safely join the thread without using `.unwrap()` as it could `panic` if error in thread
  // Safety handle the join result with `match`
  match spawned_thread.join() {
    Ok(_) => println!(
      "Thread finished successfully!",
    ),
    Err(e) => eprintln!(
      "Error joining thread: {:?}",
      e
    ),
  }

  // OR safety handle the join result with `if let`
  /*
  if let Err(e) = spawned_thread.join() {
    println!(
      "Error joining thread: {:?}",
      e
    );
  } else {
    println!(
      "Thread finished successfully!"
    );
  }
  */

  for num in 0..10 {
    println!(
      "I am watching Naruto in open space {} of the mangakissa without any sound.",
      num
    );
  }
}




// Example 4
// Channels: threading but passing messages from threads to other threads using channels
use std::sync::mpsc;


fn print_type_of<T>(_: &T) {
  println!("{}", std::any::type_name::<T>());
}

pub fn thread_message_passing(first_name: &str) {
  let (transmitter, receiver) = mpsc::channel();

  // need to own the value here and have the owned value moved in the thread so compiler won't
  // cry because function scope lifetime is smaller than the thread on
  let name = String::from(
    first_name
  );

  thread::spawn(
    move || {
      // OR use static lifetimes reference like : `first_name: &'static str` 
      // and declare here the `name` variable so that its lifetime is linked to the
      // thread scope and not the function one
      // `let name = String::from(first_name);`
      let sent_name = transmitter
          .send(name);
      // using `if let` way to handle error
      // just printing type
      print_type_of(&sent_name);
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
    }
  );

  // this is returning a `Result` an
  let received_name = receiver.recv();
  // printing type
  print_type_of(&received_name);
  // println!("{:?} left the mangakissa at 00:03:43.", her_name);
  // using `match` way to handle error
  match received_name {
    Ok(data) => println!(
      "{:?} left the mangakissa at 00:03:43", 
      data
    ),
    Err(e) => println!(
      "Error joining thread: {:?}",
      e
    ),
  }
}
*/

// Example 5:
// // using more cored to compute a result in a quicker way using "Multiple" transmitters and "One"
// receiver.
// Always one receiver but producers can be multiple. We use that here to make heavy calculations
// that the most powerful quantum computer on earth can't do but Rust can!


use std::sync::mpsc;


pub fn multithread_parallel_produce_and_receive_once_to_calculate_result(v: &Vec<u64>) {

  let (transmitter, receiver) = mpsc::channel();

  // spawn a thread for each element in the vector
  for i in 0..v.len() {
    // clone the transmitter for each thread
    let tx_clone = transmitter.clone();
    // get the current number from the vector
    let value = v[i];

    thread::spawn(
      move || {
        // simulate a calculation that no one can do
        let result = value * 2123499;
        // send the result via channel and handle error using `if let`
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
      }
    );
  }

  // initial `transmitter` never been used as we just cloned it in the for loop, therefore, we
  // need to drop it here otherwise `receiver` will be still waiting to receive something...
  drop(transmitter);

  // prepare this variable to collect the sum of the results
  let mut total: u64 = 0;

  // receive reuslts fromt he threads one by one and will push thiose one by one to the
  // `result_vec`
  // ensure we reveive the correct number of results by using for loop with `v.len()`
  for _ in 0..v.len() {
    // handle error using the `match` method
    match receiver.recv() {
      // collect the received data summing it up in `total`
      Ok(data) => total += data,
      // handle receiving errors
      Err(e) => println!(
        "Error receiving data: {:?}",
        e
      ),
    }
  }
  // print the total sum
  println!(
    "Total: {}",
    total
  );
}







