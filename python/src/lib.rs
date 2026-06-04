use kitsuyui_rust_playground_lib as kitsuyui_rust_playground;
use pyo3::prelude::*;

#[pyfunction]
fn multiply_sum(x: i64, y: i64, factor: i64) -> PyResult<String> {
    Ok(kitsuyui_rust_playground::multiply_sum(x, y, factor))
}

#[pymodule]
#[pyo3(name = "_native")]
fn python_kitsuyui_rust_playground(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(multiply_sum, m)?)?;
    Ok(())
}
