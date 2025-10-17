use pyo3::prelude::*;

#[pyfunction]
pub fn convert(filename: String) -> PyResult<()> {
    png2svg_core::convert(filename).map_err(|e| {
        pyo3::exceptions::PyRuntimeError::new_err(format!("Failed to convert PNG to SVG: {}", e))
    })
}

#[pyfunction]
pub fn convert_parallel(filenames: Vec<String>) -> PyResult<()> {
    png2svg_core::convert_parallel(filenames).map_err(|e| {
        pyo3::exceptions::PyRuntimeError::new_err(format!("Failed to convert PNG to SVG: {}", e))
    })
}

#[pyfunction]
pub fn convert_directory(directory: &str) -> PyResult<()> {
    png2svg_core::convert_directory(directory).map_err(|e| {
        pyo3::exceptions::PyRuntimeError::new_err(format!("Failed to convert PNG to SVG: {}", e))
    })
}
