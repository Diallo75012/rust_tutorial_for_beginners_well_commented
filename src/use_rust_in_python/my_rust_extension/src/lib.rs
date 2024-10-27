// everything here was added by default
// it is not me hahaha
use pyo3::prelude::*;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

// let's add our own little function
// we need a decorator `#[pyfunction]` above
// our Rust function to be able to call it from Python
#[pyfunction]
fn double_cake_parts(cake_part: i32) -> i32 {
  cake_part * 2
}

/// A Python module implemented in Rust.
#[pymodule]
// this module name has to be same as 
// project folder name `my_rust_extension`
fn my_rust_extension(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    // if we had more function we would need to add those here using that same synthaxe as in the boilerplate or maybe use a list and iterator... mayyyybe...
    m.add_function(wrap_pyfunction!(double_cake_parts, m)?)?;

    Ok(())
}
