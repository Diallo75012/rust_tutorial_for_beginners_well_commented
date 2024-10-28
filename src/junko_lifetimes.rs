// we are going to start with unique lifetime eg.
// after we will be going to explain some complexe ones
// and more in between but the hands-on is not
// going to be heavy this time... Reeeelax!

// let introduce lifetimes to this function
// "'a" is here to make sure that
// the shortest scope reference is 
// how long this function will track references
#[allow(dead_code)]
fn longest_wtih_lifetime<'a> (
    x: &'a str, y: &'a str
  ) -> &'a str {
  if x.len() > y.len() {
    x
  } else {
    y
  }
}

// we are alos going here to use different scope
// just to show what is "scope" and it will
// explain why "lifetimes" are sometimes necessary
// rust automatically infers lifetimes 
// but there are some limits to that
// eg: custom Types using "&str" references..
// here you got an example of `inferred` lifetimes
/*
pub fn lifetimes_different_scope() {
  let string1 = String::from(
    "'When the problen happened Naruto was displayed but no sound could be heard'"
  );
  
  // we are use `{}` to create another scope inside
  // the function. Outside of `{}` we are in the 
  // function's "scope", this is a nested scope..
  {
    // inner scope
    // "OWNED" `string2` variable
    let string2 = String::from(
      "'No problem happened, just the cabin...'"
    );
    let result = longest_wtih_lifetime(
      // but in the inner nested scope we have
      // access to the function general scope
      // `string1` which is here just borrowed
      string1.as_str(),
      string2.as_str()
    );
    println!(
      "Result inside nested scope, The longest string is: {}", 
       result
    );
    println!(
      "String2 inside nested scope: {:?}", 
      string2.clone()
    );
  }

  // outside of the nested scope
  // we get error: 
  // "a local variable with a similar name exists:" 
  // "`string1`"
  // because string has been defined in the 
  // inner nested scope, we can't access to it
  // when trying to print in the general `fn` scope
  
  println!(
    "String2 inside nested scope: {:?}",
    string2.clone()
  );
  
  println!(
    "String1 still accessible as only borrowed in the nested scope: {:?}",
    string1.clone()
  );

 

}
*/


// Example 2: struct introduction 
// and more complexity with multi-lifetimes

// we have created struct with implementation
// as normal but... 
// let's now transform it with different lifetimes
// for each fields
struct Doctor<'a, 'b, 'c> {
  name: &'a str,
  speciality: &'b str,
  hospital: &'c str,
}

impl<'a, 'b, 'c> Doctor<'a, 'b, 'c> {
  fn print_info(&self) {
    println!(
      "Doctor: {}\n Speciality: {}\n Hospital: {}",
      self.name, self.speciality, self.hospital
    );
  }
}

#[allow(dead_code)]
pub fn multiple_lifetimes_to_struct() {
  // those x3 variable are `&str` = "string literals"
  // not "String". be careful with that...
  let doctor_name = "Dr. Junko";
  let doctor_speciality = "Cardialogy in Manga Kissa Only!";
  let doctor_hospital = "Shibuya General Hospital";

  // let's now create a `Doctor` instance 
  // using those `&str` fields
  let doctor = Doctor {
    name: doctor_name,
    speciality: doctor_speciality,
    hospital: doctor_hospital,
  };

  // General scope where `Doctor` instance exists
  // we expect to have all fields printed
  doctor.print_info();

  // nested scope
  {
    // we are going to consume here `speciality` field
    // so here the `&str` will be consummed by println! macro, miam miam!
    println!(
      "Speciality (from nested scope): {}", 
      doctor_speciality
    );
  }
 
  // some anothers prints after the inner scope  
  // we keep those for after
  // so what happened? 
  // why did we get an error in the previous 
  // example but here it works fine?
  // BORROWED!!!!
  // println! macro is consuming 
  // a "borrowed" "hospital" field
  println!(
    "Hospital (after nested scope): {}", 
    doctor.hospital
  );
  // println! macro is consuming 
  // a "borrowed" "speciality" field
  println!(
    "Speciality (after nested scope): {}", 
     doctor.speciality
  );

  // therefore here it will print in inner scope
  // and function general scope as it is borrowed
  // in the previous example the variable was "owned"
  
}

//last example: scope limited
pub fn multiple_lifetimes_to_struct_scope_limited() {
  // <'a> still lives
  // you didn't guess right.... "Name" is avaibale but....
  let name = "Dr. Junko";

  {
    let hospital = "Shibuya General Hospital";
    {
      let speciality = "Cardiology";

      let doctor = Doctor {
        name,
        speciality,
        hospital,
      };
      // this one should be OK
      println!(
        "<Inside nested2 scope>Name: {}",
        doctor.name
      );
    } // end of nested2
    // this one should be OK!
    // because available in same scope
    println!(
      "<Inside nested1 scope> Hospital: {}",
      hospital
    );
    // here "Problem" as its lifetime will die
    // after the nestes2 scope 
    // where it has been defined
    // we comment it out and then we check it
    // as epected the reference doesn't exist anymore
    // = bye bye!    
    /*
    println!(
      "<Inside nested1 scope> Speciality: {}",
      doctor.speciality
    );
    */

  } // end of nested1
  // after the inner scope bye bye <'b, 'c> lifetimes
  
  // now let's try to print again speciality 
  // for the general fucntion scope
  // you should be able to guess...
  // answer: this might create an error....
  // therefore, here the `&str` is pointing to nothing
  // therefore, error in your face!
  /*
  println!(
    "<Outside nested scope>Speicality: {}",
    doctor.speciality
  );
  // Outputs:
  // "doctor.speciality"
  // "        ^^^^^^ not found in this scope"

  // <'a> still lives... as it has been defined
  // in the general scope, so it's lifetime is
  // decoupled from the other 2 define
  // in inner nested scope
  println!(
    "<Outside nested scope>Name: {}",
     // not "doctor.name" as doctor has been defined
     // in the nested scope
     doctor.name
  );
  */
  //println!("is name still available{}", name)
}
