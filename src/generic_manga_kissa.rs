// Example 1: `Struct` with `Enum` Fields
// to be able to have several Types

use std::fmt::Debug;



// we don't use the `Enum` in this example
// so this will allow clean terminal print without warnings for `dead_code`
#[allow(dead_code)]
#[derive(Debug)]
pub enum MangaKissaType {
  // description of issue
  Issue(String),
  // request a feature (description)
  FeatureRequest(String),
  // improvement suggestion with details and priority
  Improvement {
    details: String,
    priority: u8
  }
}

/*
#[allow(dead_code)]
#[derive(Debug)]
pub struct Ticket {
  id: i32,
  name: String,
  // using the `Enum` here allows us to handle different scenarios
  // it is still limited to the `Enum` Fields
  // but helps to prevent having huge `struct`
  // don't forget that all Fields need to be defined
  // when instantiating a `struct`
  manga_kissa_type: MangaKissaType,
}
*/

// example 2" Now the same but using Generics
// when using generics we do not need the `Enum` anymore
// the `struct` also becomes super powerful
// as now it is "type-agnostic"
// we can now have different tickets with different types and add tickets
// better for our codebase if the marketing team
// decides to implement some more tickets
// so here gain in scalability and reduce complexity
#[allow(dead_code)]
#[derive(Debug)]
pub struct Ticket<T> {
  // should be u32` not `i32` like in previous example
  // ids are positive only
  id: u32,
  name: String,
  // Generic field, type can be any, no need `Enum`
  data: T,
}



pub fn manage_any_manga_kissa_type() {
  /*
  // example 1
  let bug_ticket = Ticket {
    id: 1,
    name: String::from(
      "Shibuya Kokoro Manga Kissa Level 7."
    ),
    manga_kissa_type: MangaKissaType::Issue(
      String::from(
        "Soundless anime playing on screen, can't hear anything!"
      )
    ),
  };

  let feature_ticket = Ticket {
    id: 2,
    name: String::from(
      "Help Tourist Find They Way In Manga Kissa Labyr."
    ),
    manga_kissa_type: MangaKissaType::FeatureRequest(
      String::from(
        "Have interactive map for people to know where they are."
      )
    ),
  };


  let improvement_ticket = Ticket {
    id: 3,
    name: String::from(
      "Screen Display of Anime."
    ),
    manga_kissa_type: MangaKissaType::Improvement {
      details: String::from(
        "Optimize image for better rendering and immersion into the anime art."
      ),
      priority: 2,
    },
  };

  println!("Bug ticket: {:?}", bug_ticket);
  println!("Feature ticket: {:?}", feature_ticket);
  println!("Improvement ticket: {:?}", improvement_ticket);


  // example 1 Bis: With Pattern Matching
  let special_ticket = Ticket {
    id: 1,
    name: String::from("Add new login feature"),
    manga_kissa_type: MangaKissaType::FeatureRequest(
      String::from("Users requested a new login feature")
    ),
  };

  handle_ticket(&special_ticket);
  

  // example 2
  
  let bug_ticket = Ticket {
    id: 1,
    name: String::from("Shibuya Kokoro Manga Kissa Level 7."),
    // data is a `String`
    data: "Soundless anime playing on screen, can't hear anything!"
      .to_string(),
  };

  let feature_ticket = Ticket {
    id: 2,
    name: String::from(
      "Help Tourists Find They Way In Manga Kissa Labyr."
    ),
    // data is a `Vec<&str>`
    data: vec![
      "User request", "Sales Department", "Marketing Department"
    ].to_vec(),
  };

  let improvement_ticket = Ticket {
    id: 3,
    name: String::from(
      "Screen display of anime."
    ),
    // let do a tuple here
    data: (
      String::from(
        "Optimize image for better rendering and immersion into the anime art."
      ),
      2
    ),
  };

  println!("{:?}", bug_ticket);
  println!("{:?}", feature_ticket);
  println!("{:?}", improvement_ticket)
  */

  // example 2 Bis: No more pattern matching needed
  let improvement_ticket = Ticket{
    id: 3,
    name: String::from(
      "Mori Tower Rigoleto Italian Restaurant."
    ),
    data: (
      String::from(
        "Have a VIP table always available for creditizens on Saturday night."
      ),
      // very high priority is 1
      1
    ),
  };

  // works with any `Ticket<T>` 
  // with T implementing Debug
  process_ticket(&improvement_ticket)
}

/*
// example 1 Bis
fn handle_ticket(ticket: &Ticket) {
  match &ticket.manga_kissa_type {
    MangaKissaType::Issue(description) =>
      println!(
        "Handling issue: {}", description
      ),
    MangaKissaType::FeatureRequest(description) =>
      println!(
        "Handling feature request: {}", description
      ),
    MangaKissaType::Improvement {
      details, priority } => {
        println!(
          "Handling improvement: {}; with priority {}", 
           details,
           priority
        )
      }        
    }
}
*/


// example 2 Bis
// we can use ticket directly
// we do not need pattern matching anymore
// as each ticket has its own data type
// we cannot mistakenly use wrong type for data
// once the ticket is created, "Reducing errors"
fn process_ticket<T: Debug>(
    ticket: &Ticket<T>
  ) {
    println!(
      "Processing Ticket ID: {}, Name: {}",
      ticket.id, ticket.name
    );

  println!("Ticket Data: {:?}", ticket.data)
}



