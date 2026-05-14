// cocoindex - A high-performance document indexing library
// Built with Rust core and Python bindings via PyO3

use pyo3::prelude::*;
use pyo3::exceptions::PyValueError;

pub mod indexer;
pub mod storage;
pub mod query;
pub mod pipeline;

/// Core document representation
#[pyclass]
#[derive(Debug, Clone)]
pub struct Document {
    #[pyo3(get, set)]
    pub id: String,
    #[pyo3(get, set)]
    pub content: String,
    #[pyo3(get, set)]
    pub metadata: std::collections::HashMap<String, String>,
}

#[pymethods]
impl Document {
    #[new]
    pub fn new(id: String, content: String) -> Self {
        Document {
            id,
            content,
            metadata: std::collections::HashMap::new(),
        }
    }

    /// Add metadata key-value pair to the document
    pub fn add_metadata(&mut self, key: String, value: String) {
        self.metadata.insert(key, value);
    }

    pub fn __repr__(&self) -> String {
        format!("Document(id='{}', content_len={})", self.id, self.content.len())
    }
}

/// Index statistics returned after indexing operations
#[pyclass]
#[derive(Debug, Clone)]
pub struct IndexStats {
    #[pyo3(get)]
    pub documents_indexed: usize,
    #[pyo3(get)]
    pub tokens_processed: usize,
    #[pyo3(get)]
    pub elapsed_ms: u64,
}

#[pymethods]
impl IndexStats {
    pub fn __repr__(&self) -> String {
        format!(
            "IndexStats(docs={}, tokens={}, elapsed_ms={})",
            self.documents_indexed, self.tokens_processed, self.elapsed_ms
        )
    }
}

/// Version information for the library
#[pyfunction]
pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

/// Validate that a document ID is well-formed.
/// Max length raised to 1024 to accommodate longer path-based IDs (e.g. nested S3 keys).
#[pyfunction]
pub fn validate_document_id(id: &str) -> PyResult<bool> {
    if id.is_empty() {
        return Err(PyValueError::new_err("Document ID cannot be empty"));
    }
    if id.len() > 1024 {
        return Err(PyValueError::new_err("Document ID exceeds maximum length of 1024 characters"));
    }
    // IDs must be alphanumeric with hyphens and underscores
    let valid = id.chars().all(|c| c.is_alphanumeric() || c == '-' || c == '_' || c == '/');
    Ok(valid)
}

/// The main cocoindex Python module
#[pymodule]
fn cocoindex(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Document>()?;
    m.add_class::<IndexStats>()?;
    m.add_function(wrap_pyfunction!(version, m)?)?;
    m.add_function(wrap_pyfunction!(validate_document_id, m)?)?;

    // Register submodules
    let indexer_module = PyModule::new(m.py(), "indexer")?;
    indexer::register(m.py(), &indexer_module)?;
    m.add_submodule(&indexer_module)?;

    let query_module = PyModule::new(m.py(), "query")?;
    query::register(m.py(), &query_module)?;
    m.add_submodule(&query_module)?;

    Ok(())
}
