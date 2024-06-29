use kitsuyui_rust_playground_lib as kitsuyui_rust_playground;
use pyo3::prelude::*;

#[pyfunction]
fn my_calc(a: i64, b: i64, c: i64) -> PyResult<String> {
    Ok(kitsuyui_rust_playground::my_calc(a, b, c))
}

#[pymodule]
#[pyo3(name = "kitsuyui_rust_playground")]
fn python_kitsuyui_rust_playground(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(my_calc, m)?)?;
    Ok(())
}
