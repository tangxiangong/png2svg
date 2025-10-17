mod ffi;

use pyo3::prelude::*;

/// A Python module implemented in Rust. The name of this function must match
/// the `lib.name` setting in the `Cargo.toml`, else Python will not be able to
/// import the module.
#[pymodule]
fn _core(m: &Bound<'_, PyModule>) -> PyResult<()> {
    rayon::ThreadPoolBuilder::new()
        .num_threads(num_cpus::get())
        .build_global()
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyRuntimeError, _>(e.to_string()))?;

    m.add_function(wrap_pyfunction!(ffi::convert, m)?)?;
    m.add_function(wrap_pyfunction!(ffi::convert_parallel, m)?)?;
    m.add_function(wrap_pyfunction!(ffi::convert_directory, m)?)?;
    Ok(())
}
