// let use the example that we have viewed earlier and extend it a bit
use std::fs;
use std::io::Error;
use std::fmt;


pub struct SecretFolder {
  pub secret_file: String,
}

impl SecretFolder {
  // first way using `?`
  // if we omit the `&self` the struct won;t be discovered
  // the `Error` from `std::io::Error` implements `Debug`
  /*
  fn question_mark(&self, path: &str) -> Result<String, Error> {
    let secret_story = fs::read_to_string(path)?;
    // `Result` return `Ok()` or `Err()`, here `Ok()` is a `String` 
    Ok(secret_story)
  }

  // second way using `match`
  fn another_secret(&self, path: &str) -> Result<String, Error> {
    match fs::read_to_string(path) {
      Ok(the_super_secret) => Ok(the_super_secret),
      Err(e) => Err(e),
    }
  }
  */

  // third way using `.unwrap()` just for devs prototypes not prodution
  fn dev_err(&self, path: &str) -> Result<String, Error> {
    let dev_mistakes_check = fs::read_to_string(path).unwrap();
    Ok(dev_mistakes_check)
  }

}

// lets implement `Display` for the struct like in previous video
impl fmt::Display for SecretFolder {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "Error Secret Not Accessible At: {}", self.secret_file)
  }
}


// let's now make this public function to play with those
pub fn so_many_errors() -> Result<String, Error> {
  /*
  // let's instantiate x3 times the struct
  let secret_in_file1 = SecretFolder {
    secret_file: "./files/first_secret.txt".to_string()
  };

  let secret_in_file2 = SecretFolder {
    secret_sfile: "./files/second_secret.txt".to_string()
  };
  */

  let secret_in_file3 = SecretFolder {
    secret_file: "./files/dev_secret.txt".to_string()
  };
  println!("{}", secret_in_file3);
  // we create variable to return it, here either of these two `<String, Error>`
  // let err_or_not = secret_in_file3.dev_err(&secret_in_file3.secret_file);
  // err_or_not

  let file_content = fs::read_to_string(&secret_in_file3.secret_file);
  match file_content {
    Ok(content) => Ok(content),
    Err(_e) => secret_in_file3.dev_err(&secret_in_file3.secret_file),
  }
}









