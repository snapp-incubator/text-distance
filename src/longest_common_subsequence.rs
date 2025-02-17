use pyo3::prelude::*;


#[pyfunction]
fn lcsseq(s1: &str, s2: &str) -> String {
    let chars1: Vec<char> = s1.chars().collect();
    let chars2: Vec<char> = s2.chars().collect();
    let n = chars1.len();
    let m = chars2.len();

    // Create a DP table
    let mut dp = vec![vec![0; m + 1]; n + 1];

    // Fill the DP table
    for i in 1..=n {
        for j in 1..=m {
            if chars1[i - 1] == chars2[j - 1] {
                dp[i][j] = dp[i - 1][j - 1] + 1;
            } else {
                dp[i][j] = dp[i - 1][j].max(dp[i][j - 1]);
            }
        }
    }

    // Reconstruct the LCS from the DP table
    let mut lcs = String::new();
    let (mut i, mut j) = (n, m);

    while i > 0 && j > 0 {
        if chars1[i - 1] == chars2[j - 1] {
            lcs.push(chars1[i - 1]);
            i -= 1;
            j -= 1;
        } else if dp[i - 1][j] > dp[i][j - 1] {
            i -= 1;
        } else {
            j -= 1;
        }
    }

    lcs.chars().rev().collect()
}

#[pyfunction]
pub fn longest_common_subsequence_max(
    s1: Vec<String>,
    s2: Vec<Vec<String>>,
) -> Vec<usize> {
    let mut results = Vec::new();

    for (i, query) in s1.iter().enumerate() {
        if i < s2.len() {
            let mut max_result = ("".to_string(), 0); // Initialize the max result for the current sublist
            for candidate in &s2[i] {
                let lcs = lcsseq(query, candidate);
                let length = lcs.len();
                if length > max_result.1 {
                    max_result = (lcs, length); // Update max if the current LCS is longer
                }
            }
            results.push(max_result.1);
        }
    }

    results
}
