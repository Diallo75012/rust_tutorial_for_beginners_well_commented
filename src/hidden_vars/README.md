## Reading environment variables

### install `dotenv`
```bash
cargo add dotenv
```

### create a `.env` file in the `src/` dir
echo TEST_ENV_VAR=test_env_var_from_src_dir > $HOME/<project_root>/src/.env

### import `dotenv` and standard library `env`
```rust
use dotenv::dotenv;
use std::env;
```

### read env vars
```rust
let read_env_var = env::var("TEST_ENV_VAR");
```

### use `match` to do logic, here we are just testing to read existant env var
```rust
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


```
