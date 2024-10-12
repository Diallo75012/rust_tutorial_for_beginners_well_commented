// we import the file system reader
use std::fs;
use std::io;
use std::fmt;

// this will be to save a path
pub struct SecretFolder {
  secret_file: String,
}

// this will hold different functions to see different
// ways of handling those errors using our previously explained example
#[allow(dead_code)]
#[allow(unused_variables)]
#[allow(non_snake_case)]
impl SecretFolder {
  // firt way using `?`
  fn question_mark(&self, path: &str) -> Result<String, io::Error> {
    let secret_story = fs::read_to_string(path)?;
    Ok(secret_story)
  }

  // second way using `match`
  fn AnotherSecretFolder(&self, path: &str) -> Result<String, io::Error> {
    match fs::read_to_string(path) {
      Ok(the_super_secret) => Ok(the_super_secret),
      Err(e) =>  Err(e),
    }
  }

  // third way: just for dev side of app/prototypes
  fn dev_err(&self) {
    let dev_mistake = fs::read_to_string("./files/err_handling_dev_mistake_unwrap.txt").unwrap();
  }
}

// display implementation as we have seen in previous video
impl fmt::Display for SecretFolder {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "Error Secret Not Accessible Because: {}", self.secret_file)
  }
}

// our public func
pub fn so_many_errors() -> Result<String, io::Error> {
  // instantiate structs
  /*
  let secret_in_folder1 = SecretFolder {
    secret_file: "./files/first_secret".to_string()
  };
 
  let secret_in_folder2 = SecretFolder {
    secret_file: "./files/second_secret".to_string()
  };
  */ 

  let secret_in_folder3 = SecretFolder {
    secret_file: "./files/dev_secret".to_string()
  };

  
  // just return the the variable defined
  let err_or_not = secret_in_folder3.question_mark(&secret_in_folder3.secret_file);
  err_or_not
}


