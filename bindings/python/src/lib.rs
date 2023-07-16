use pyo3::prelude::*;
use libgzip_search_rs::by_gzip;

#[pyfunction]
fn gzip_search(query: &str, candidate_chunks: Vec<String>, top_k: usize) -> Vec<(String, f32)> {
    by_gzip::search(
        query, &candidate_chunks, top_k
    )
}

#[pymodule]
#[pyo3(name = "libgzip_search")]
fn python(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(gzip_search, m)?)?;
    Ok(())
}