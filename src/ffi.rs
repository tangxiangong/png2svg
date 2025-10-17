use pyo3::prelude::*;

#[pyfunction]
pub fn convert(filename: String, output_dir: Option<String>) -> PyResult<()> {
    png2svg_core::convert(filename, output_dir).map_err(|e| {
        pyo3::exceptions::PyRuntimeError::new_err(format!("Failed to convert PNG to SVG: {}", e))
    })
}

#[pyfunction]
pub fn convert_parallel(filenames: Vec<String>, output_dir: Option<String>) -> PyResult<()> {
    png2svg_core::convert_parallel(filenames, output_dir).map_err(|e| {
        pyo3::exceptions::PyRuntimeError::new_err(format!("Failed to convert PNG to SVG: {}", e))
    })
}

#[pyfunction]
pub fn convert_directory(directory: &str, output_dir: Option<String>) -> PyResult<()> {
    png2svg_core::convert_directory(directory, output_dir).map_err(|e| {
        pyo3::exceptions::PyRuntimeError::new_err(format!("Failed to convert PNG to SVG: {}", e))
    })
}
