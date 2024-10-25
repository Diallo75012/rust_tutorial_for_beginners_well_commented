//use mistralai_client::v1::client::Client;
use mistralai_client::v1::{
    chat::{ChatMessage, ChatMessageRole, ChatParams},
    client::Client,
    constants::Model,
};


pub fn call_mistral_llm() {
  /* 
  // Example with `.unwrap()`
  // we have used the method: export MISTRAL_API_KEY==*******************
  //let api_key = "your_api_key";
  //let client = Client::new(Some(api_key), None, None, None).unwrap();
  let client = Client::new(None, None, None, None).unwrap();

  let model = Model::OpenMistral7b;
  let messages = vec![ChatMessage {
    role: ChatMessageRole::User,
    content: "What is the biggest caital city in Asia?".to_string(),
    tool_calls: None,
  }];
  let options = ChatParams {
    temperature: 0.0,
    random_seed: Some(53),
    ..Default::default()
  };

  let result = client.chat(model, messages, Some(options)).unwrap();

  println!("Assistant: {}", result.choices[0].message.content);
  */

  // Example without `unwrap()` but pattern matching
  let client_result = Client::new(None, None, None, None);

  match client_result {
    Ok(client) => {
      let model = Model::OpenMistral7b;
      let messages = vec![ChatMessage {
        role: ChatMessageRole::User,
        content: "What is the biggest caital city in Asia?".to_string(),
        tool_calls: None,
      }];
      let options = ChatParams {
        temperature: 0.0,
        random_seed: Some(53),
        ..Default::default()
      };
      // If the client was successfully created, proceed with listing models
      match client.chat(model, messages, Some(options)) {
        Ok(result) => {
          if let Some(response) = result.choices.get(0) {
          println!("Result: {:?}", result);
          if response.message.content.len() > 0 {
            println!("Response is: {:?}", response.message.content);
          } else {
            // this is like the `None` side
            println!("No response found in the data returned. Result was: {:?}", result);
          }
          }
        }
        Err(error_response) => {
          // Handle the error that occurred while listing models
          println!("Error Response: {:?}", error_response);
        }
      }
    }
    Err(problem_with_client) => {
      // Handle the error that occurred while creating the client
      println!("Problem with Client: {:?}", problem_with_client);
    }
  }
}


pub fn get_list_mistral_models() {

 
  // emaple using `.unwrap()`
  let client = Client::new(None, None, None, None).unwrap();
  let result = client.list_models().unwrap();
  println!("First Model ID: {:?}", result.data[0].id);
 

  /*
  // Example without `.unwrap()` using patterm mathcing
  // Assume `client` can potentially return an error, wrap in `Result`
  // same function but not using `unwrap()` but only pattern mathcing on `Result`
  // and considered as `Option` as `response` can be `None` and we want to handle that
  let client_result = Client::new(None, None, None, None);

  match client_result {
    Ok(client) => {
      println!("{:?}", client.list_models().unwrap().data);
      // If the client was successfully created, proceed with listing models
      match client.list_models() {
        Ok(result) => {
          
          // If listing models was successful = like an `Option` with Some() and None sides
          if let Some(result) = &result.data[0].root {
            println!("List of Models available: {:?}", result);
            } else {
                // this is like the `None` side
                println!("No models found in the response data. Result was: {:?}", result);
            }
          }
        Err(error_response) => {
          // Handle the error that occurred while listing models
          println!("Error Response: {:?}", error_response);
        }
      }
    }
    Err(problem_with_client) => {
      // Handle the error that occurred while creating the client
      println!("Problem with Client: {:?}", problem_with_client);
    }
  }
  */
}

