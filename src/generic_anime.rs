/*
// Example 1: Struct with an Enum Fields 
// to be able to have several Types
#[allow(dead_code)]
#[derive(Debug)]
pub enum TicketType {
  // description of issue 
  Issue(String),
  // requets a feature with description
  FeatureRequest(String),
  // improvement suggestion with details and priority
  Improvement {
    details: String,
    priority: u8
  },
}

// Example 1: Struct with an Enum Fields 
// to be able to have several Types
#[allow(dead_code)]
#[derive(Debug)]
pub struct Ticket {
  pub id: u32,
  pub title: String,
  // using the `Enum` here allows us to handle different scenarios
  // it is still limited to the `Enum` existing Fields,
  // but helps to prevent having a huge `Struct`
  // don't forget that all Fields need to be defined when instantiating the `Struct`
  pub ticket_type: TicketType,
}
*/

// Example 2: Now The Same but using Generics
// when usig generics we do not need the Enum anymore
// the struct also becomes super powerful 
// as now it is type-agnostic
// we can now have different tyckets 
// with different types and add tickets
// better for our codebase if the marketing team 
// decides to implement some more tickets
// so here we gain in scalability 
// and reduce the complexity
#[allow(dead_code)]
#[derive(Debug)]
pub struct Ticket<T> {
  id: u32,
  title: String,
  // Generic field, type can be any
  data: T,
}


pub fn manage_any_ticket_type() {
  /*
  // Example 1: Struct with an Enum Fields 
  // to be able to have several Types


  let bug_ticket = Ticket {
    id: 1,
    title: String::from("App crashes on login"),
    ticket_type: TicketType::Issue(
      String::from(
        "Critical bug affecting login"
      )
    ),
  };

  let feature_ticket = Ticket {
    id: 2,
    title: String::from("Add dark mode"),
    ticket_type: TicketType::FeatureRequest(
      String::from( 
        "User requested a dark theme"
      )
    ),
  };

  let improvement_ticket = Ticket { 
    id: 3,
    title: String::from("Improve load time"),
    ticket_type: TicketType::Improvement {
      details: String::from(
        "Optimize image loading"
      ),
      priority: 2,
    },
  };

  println!("Bug ticket: {:?}", bug_ticket);
  println!("Feature ticket: {:?}", feature_ticket);
  println!("Improvement ticket: {:?}", improvement_ticket);
  */  

  /*
  // Example 1 Bis: With pattern matching
  let ticket = Ticket {
    id: 1,
    title: String::from("Add new login feature"),
    ticket_type: TicketType::FeatureRequest(String::from("Users requested a new login feature")),
  };

  handle_ticket(&ticket)
  */


  /*
  // Example 2:
  // Creating tickets with 
  // different data types using generics
  let bug_ticket = Ticket {
    id: 1,
    title: String::from("App crashes on login"),
    // Data is a `String`
    data: "Critical bug affecting login"
      .to_string(),
  };

  let feature_ticket = Ticket {
    id: 2,
    title: String::from("Add dark mode"),
    // Data is a `Vec<&str>`
    data: vec![
      "User request", "Marketing"
    ].to_vec(),
  };

  let improvement_ticket = Ticket {
    id: 3,
    title: String::from("Improve load time"),
    // Data is a `Tuple`
    data: (
      String::from("Optimize image loading"),
      2
    ),
  };

  println!("{:?}", bug_ticket);
  println!("{:?}", feature_ticket);
  println!("{:?}", improvement_ticket);
  */

  // Example 2 Bis
  let improvement_ticket = Ticket {
    id: 3,
    title: String::from("Improve load time"),
    data: (
      String::from("Optimize image loading"), 
      2
    ),
  };

  // Works for any Ticket<T> with T 
  // implementing Debug
  process_ticket(&improvement_ticket)

}


/*
// Example 1 Bis: With Pattern Matching
pub fn handle_ticket(ticket: &Ticket) {
  match &ticket.ticket_type {
    TicketType::Issue(description) => println!("Handling issue: {}", description),
    TicketType::FeatureRequest(description) => println!("Handling feature request: {}", description),
    TicketType::Improvement { details, priority } => {
        println!("Handling improvement: {} with priority {}", details, priority)
    }
  }
}
*/

 
// Example 2: Bis - No more need pattern matching
// we can use ticket directly
// we do not need pattern matching anymore 
// as each ticket has its own data type
// cannot mistakenly use a wrong type for data
// once the ticket is created, reducing errors
fn process_ticket<T: std::fmt::Debug>(
    ticket: &Ticket<T>
  ) {
    println!(
      "Processing Ticket ID: {}, Title: {}",
      ticket.id, ticket.title
  );

  println!("Ticket Data: {:?}", ticket.data);
}
