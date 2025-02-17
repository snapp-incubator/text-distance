use std::collections::HashMap;
use pyo3::prelude::*;

// Function to calculate term frequency (TF) for a string
fn calculate_tf(text: &str) -> HashMap<String, usize> {
    let mut tf = HashMap::new();
    for word in text.split_whitespace() {
        *tf.entry(word.to_string()).or_insert(0) += 1;
    }
    tf
}

// Function to calculate cosine similarity between two term frequency maps
fn cosine_similarity_tf(tf1: &HashMap<String, usize>, tf2: &HashMap<String, usize>) -> f64 {
    let mut dot_product = 0.0;
    let mut norm1 = 0.0;
    let mut norm2 = 0.0;

    for (key, &value) in tf1 {
        if let Some(&value2) = tf2.get(key) {
            dot_product += value as f64 * value2 as f64;
        }
        norm1 += (value as f64).powi(2);
    }

    for &value in tf2.values() {
        norm2 += (value as f64).powi(2);
    }

    if norm1 == 0.0 || norm2 == 0.0 {
        0.0
    } else {
        dot_product / (norm1.sqrt() * norm2.sqrt())
    }
}

// Function to calculate cosine similarities between corresponding indices
#[pyfunction]
pub fn cosine_similarities(
    queries: Vec<String>,
    synonym_sets: Vec<Vec<String>>,
) -> Vec<f64> {
    queries
        .iter()
        .zip(synonym_sets.iter()) // Match corresponding indices
        .map(|(query, synonym_set)| {
            let query_tf = calculate_tf(query);

            synonym_set
                .iter()
                .map(|synonym| {
                    let synonym_tf = calculate_tf(synonym);
                    cosine_similarity_tf(&query_tf, &synonym_tf)
                })
                .fold(0.0_f64, |a, b| a.max(b)) // Take the max similarity
        })
        .map(|similarity| (similarity * 10000.0).round() / 10000.0) // Round to 4 decimal places
        .collect()
}
