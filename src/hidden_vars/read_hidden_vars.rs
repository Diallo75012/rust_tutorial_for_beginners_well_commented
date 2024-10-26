// import dotenv and std library env
use dotenv::dotenv;
use std::env;

// we have created .env file in root project dir and in src/ dir
// we want to know where is the .env file red from by default
pub fn reading_env_vars() {
  // load dotenv vars
  dotenv().ok();

  // we try to read the env var from the project root .env file
  let env_var_in_project_root_dir = env::var("TEST_ENV_VAR");
  match env_var_in_project_root_dir {
    Ok(var) => println!(
      "This is env var from project root: {}", 
      var
    ),
    Err(e) => println!(
      "Nothing red from project root dir: {}"
      , e
    ),
  }

  // we try to read the env var from the project "src/" dir
  let env_var_in_src_dir = env::var("TEST_SRC_ENV");
  match env_var_in_src_dir { 
    Ok(var) => println!(
      "This is env var from src/ dir: {}", 
      var
    ),
    Err(e) => println!(
      "This is env var from src/ dir: {}"
      , e
    ),
  }


}
