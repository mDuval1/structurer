use pyo3::prelude::*;


/// Splits a text into texts of specified size
#[pyfunction]
fn split_text(text: String, size: usize) -> PyResult<Vec<String>> {
    let result = text.as_bytes()
            .chunks(size)
            .map(|buf| unsafe { String::from_utf8_unchecked(buf.to_vec()) })
            .collect::<Vec<String>>();
    Ok(result)
}

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

/// A Python module implemented in Rust.
#[pymodule]
fn structurer(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(split_text, m)?)?;
    Ok(())
}