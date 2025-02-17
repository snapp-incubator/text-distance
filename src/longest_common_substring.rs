use pyo3::prelude::*;

/// Finds the longest common substring length between two strings
fn longest_common_substring_length(s1: &str, s2: &str) -> usize {
    let chars1: Vec<char> = s1.chars().collect();
    let chars2: Vec<char> = s2.chars().collect();
    let n = chars1.len();
    let m = chars2.len();

    let mut dp = vec![vec![0; m + 1]; n + 1];
    let mut max_length = 0;

    for i in 1..=n {
        for j in 1..=m {
            if chars1[i - 1] == chars2[j - 1] {
                dp[i][j] = dp[i - 1][j - 1] + 1;
                if dp[i][j] > max_length {
                    max_length = dp[i][j];
                }
            }
        }
    }

    max_length
}

/// Returns the maximum length of the longest common substring for each pair of strings

#[pyfunction]
pub fn longest_common_substring_max(
    s1: Vec<String>,
    s2: Vec<Vec<String>>,
) -> Vec<usize> {
    let mut results = Vec::new();
    for (i, str1) in s1.iter().enumerate() {
        if i < s2.len() {
            let mut max_length = 0; // Initialize max length
            for str2 in &s2[i] {
                let lcs_length = longest_common_substring_length(str1, str2);
                if lcs_length > max_length {
                    max_length = lcs_length; // Update max length if current result is better
                }
            }
            results.push(max_length); // Store the max length for the current comparison
        }
    }

    results
}
