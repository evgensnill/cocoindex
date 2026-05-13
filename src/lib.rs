//! CocoIndex - A high-performance data indexing library
//!
//! This crate provides the core Rust implementation for cocoindex,
//! exposing Python bindings via PyO3.

use pyo3::prelude::*;

pub mod index;
pub mod pipeline;
pub mod storage;
pub mod transform;
pub mod utils;

/// Core version of the cocoindex library
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Python module initialization
/// Registers all Python-accessible classes and functions
#[pymodule]
fn _cocoindex_rs(py: Python<'_>, m: &PyModule) -> PyResult<()> {
    // Register version info
    m.add("__version__", VERSION)?;

    // Register index submodule
    let index_module = PyModule::new(py, "index")?;
    index::register_module(py, index_module)?;
    m.add_submodule(index_module)?;

    // Register pipeline submodule
    let pipeline_module = PyModule::new(py, "pipeline")?;
    pipeline::register_module(py, pipeline_module)?;
    m.add_submodule(pipeline_module)?;

    // Register storage submodule
    let storage_module = PyModule::new(py, "storage")?;
    storage::register_module(py, storage_module)?;
    m.add_submodule(storage_module)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version_not_empty() {
        assert!(!VERSION.is_empty());
    }
}
