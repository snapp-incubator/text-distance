use pyo3::prelude::*;
use pyo3::wrap_pyfunction;

pub mod cosine_similarity;
pub mod jaccard_similarity;
pub mod longest_common_subsequence;
pub mod longest_common_substring;


use cosine_similarity::cosine_similarities;
use jaccard_similarity::jaccard_similarities;
use longest_common_subsequence::longest_common_subsequence_max;
use longest_common_substring::longest_common_substring_max;


#[pymodule]
fn snapptextdistance(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(cosine_similarities, m)?)?;
    m.add_function(wrap_pyfunction!(jaccard_similarities, m)?)?;
    m.add_function(wrap_pyfunction!(longest_common_substring_max, m)?)?;
    m.add_function(wrap_pyfunction!(longest_common_subsequence_max, m)?)?;

    Ok(())
}