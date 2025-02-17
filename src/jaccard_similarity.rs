use std::collections::HashSet;
use pyo3::prelude::*;

// Function to generate bigrams from a string
fn generate_bigrams(text: &str) -> HashSet<(char, char)> {
    text.chars()
        .zip(text.chars().skip(1))
        .collect()
}

// Function to calculate Jaccard similarity between two sets of bigrams
fn jaccard_similarity_bigrams(set1: &HashSet<(char, char)>, set2: &HashSet<(char, char)>) -> f64 {
    let intersection: HashSet<_> = set1.intersection(set2).cloned().collect();
    let union: HashSet<_> = set1.union(set2).cloned().collect();

    if union.is_empty() {
        0.0
    } else {
        intersection.len() as f64 / union.len() as f64
    }
}

// Function to calculate Jaccard similarities for corresponding indices
#[pyfunction]
pub fn jaccard_similarities(
    queries: Vec<String>,
    synonym_sets: Vec<Vec<String>>,
) -> Vec<f64> {
    queries
        .iter()
        .zip(synonym_sets.iter()) // Match corresponding indices
        .map(|(query, synonym_set)| {
            let query_bigrams = generate_bigrams(query);

            synonym_set
                .iter()
                .map(|synonym| {
                    let synonym_bigrams = generate_bigrams(synonym);
                    jaccard_similarity_bigrams(&query_bigrams, &synonym_bigrams)
                })
                .fold(0.0_f64, |a, b| a.max(b)) // Take the max similarity
        })
        .map(|similarity| (similarity * 10000.0).round() / 10000.0) // Round to 4 decimal places
        .collect()
}