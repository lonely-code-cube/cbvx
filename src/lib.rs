use pyo3::prelude::*;

pub mod spotify;

/// This is a test function that returns the given string.
#[pyfunction]
fn echo(a: &str) -> PyResult<&str> {
    Ok(a)
}

#[pymodule]
fn cbvx(py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(echo, m)?)?;

    // The spotify submodule contains functions to generate spotify image
    let spot_mod = PyModule::new(py.clone(), "spotify")?;
    spot_mod.add_function(wrap_pyfunction!(spotify::pallet, spot_mod)?)?;
    m.add_submodule(spot_mod)?;

    Ok(())
}