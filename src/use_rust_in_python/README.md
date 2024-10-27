# Use Rust in Python


## project folder tree

1. Initial Folder Structure for Your Python Project

```bash
my_app/
├── venv/                   # Your virtual environment
├── app.py                  # Your main Python code
├── __init__.py             # Makes `my_app` a package (optional)
└── requirements.txt        # List of Python dependencies (optional)
```

2. Add Rust Code to the Python Project

```bash
my_app/
├── venv/
├── app.py
├── __init__.py
├── requirements.txt
├── rust_extension/         # New folder for Rust code
│   ├── Cargo.toml          # Rust project configuration
│   ├── pyproject.toml      # Python build configuration for Rust
│   └── src/                # Source code for the Rust library
│       └── lib.rs          # Main Rust file where you implement the functions
└── tests/                  # Optional: Python tests to validate the Rust functions
    └── test_example.py
```

## Detailed Steps to Integrate Rust Code Using Maturin

### Step 1: Set Up Your Python Project and Virtual Environment

```bash
python -m venv venv
source venv/bin/activate
```

### Step 2: Add Rust Code to the Project

#### Create a New Folder for Rust Code:

In my_app folder, create a new folder called rust_extension to keep Rust code separate from Python code.
Initialize a Rust Project in rust_extension/:

```bash
cd my_app
mkdir rust_extension
cd rust_extension
cargo init --lib
```

This creates a new Rust library with Cargo.toml and a src directory containing lib.rs.

OR

Install Maturin (Globally or in Virtual Environment) Maturin can be installed using pip or using the standalone binary.

```bash
pip install maturin
# update rust if needed OR just install it using default so just press enter
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```
So here in `STEP 3` we only need `maturin develop`

And then, Use Maturin to Create a New Rust-Python Project This step initializes a new Rust project that is configured for use with Maturin to create a Python extension module.
```bash
maturin new --bindings pyo3 my_rust_extension
cd my_rust_extension
# OR
maturin new my_rust_extension
cd my_rust_extension
```
After running the maturin new command, you will see a generated project structure similar to this:
```bash
my_rust_extension/
  ├── Cargo.toml
  ├── pyproject.toml
  ├── src/
      └── lib.rs
  ├── tests/
      └── test_my_rust_extension.py
```

#### Update the Cargo.toml File: Add PyO3 as a dependency to your Cargo.toml:

From CLI:
```bash
cargo add pyo3 --features "extension-module"
```
OR manually:
```toml
[dependencies]
pyo3 = { version = "0.16", features = ["extension-module"] }
```

#### Add pyproject.toml for Maturin: Add a pyproject.toml file in the rust_extension folder for Python packaging using Maturin:

If `pyproject.toml` is updated, it should look like that:
```bash
[project]
name = "my_rust_extension"
version = "0.1.0"
authors = ["Your Name <your.email@example.com>"]
description = "A Python module implemented in Rust."
dependencies = []

[tool.maturin]
bindings = "pyo3"
```

Anyways, you "MUST" manually update `pyproject.toml`
We need the `[build-system]` and the `[tool.maturin]`

```toml
[build-system]
requires = ["maturin>=0.12"]
build-backend = "maturin"

[project]
name = "my_rust_extension"
version = "0.1.0"
description = "A Python module implemented in Rust."
authors = ["Your Name <your.email@example.com>"]

[tool.maturin]
bindings = "pyo3"
```

#### Write Your Rust Code in src/lib.rs: Modify src/lib.rs to expose a Rust function to Python using PyO3. For example:

```rust
use pyo3::prelude::*;

#[pyfunction]
fn double_number(x: i32) -> i32 {
    x * 2
}

#[pymodule]
fn rust_extension(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(double_number, m)?)?;
    Ok(())
}
```

### Step 3: Build and Install the Rust Extension in Your Python Virtual Environment
#### Install Maturin in the virtual environment:
only if not done yet, but if already done, no need
```bash
pip install maturin
```

#### Build the Rust Extension: In the rust_extension directory, run:

We "NEED/HAVE TO" to `BUILD` the Rust extension:
```bash
maturin develop
```
This command builds the Rust code as a Python extension and installs it directly into the virtual environment. This makes it available for import in your Python code.

### Step 4: Use the Rust Function in Your Python Code

#### Import the Rust Module in app.py: You can now import and use the Rust function in app.py:

```python
import rust_extension

def main():
    result = rust_extension.double_number(5)
    print(f"The double of 5 is {result}")

if __name__ == "__main__":
    main()
```

#### Optional: Add Unit Tests in the tests/ Directory
Create the tests/ Folder:

Inside my_app, create a tests/ folder to add unit tests for your project.
Add a Python Test File (test_example.py):

```python
import rust_extension

def test_double_number():
    assert rust_extension.double_number(3) == 6
```


