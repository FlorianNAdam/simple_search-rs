//! This module provides the `IncrementalLevenshtein` struct which is designed for
//! efficiently computing Levenshtein distances and similarity scores for scenarios where
//! the 'query' string is subject to incremental changes.

use crate::levenshtein::base::{
    edit_operations, levenshtein_matrix, weighted_edit_similarity, EditOperation,
};

/// A structure for incrementally calculating Levenshtein distances and similarities.
/// This is particularly efficient when repeatedly comparing slight variations of the query
/// against a constant data string.
#[derive(Clone)]
pub struct IncrementalLevenshtein {
    query: String,
    data: String,
    matrix: Vec<Vec<usize>>,
}

impl IncrementalLevenshtein {
    /// Constructs a new `IncrementalLevenshtein` with the given query and data strings.
    /// Initializes the Levenshtein matrix based on the provided strings.
    ///
    /// # Arguments
    ///
    /// * `query` - A slice of the query string.
    /// * `data` - A slice of the data string.
    pub fn new(query: &str, data: &str) -> Self {
        Self {
            query: query.to_string(),
            data: data.to_string(),
            matrix: levenshtein_matrix(query, data),
        }
    }

    /// Private method to determine the length of the identical starting substring
    /// between the current query and a new query.
    ///
    /// # Arguments
    ///
    /// * `new_query` - A slice of the new query string to compare.
    ///
    /// # Returns
    ///
    /// A `usize` value indicating the count of identical leading characters.
    fn query_similarity(&mut self, new_query: &str) -> usize {
        let mut identical = 0;
        for (new, old) in self.query.chars().zip(new_query.chars()) {
            if new != old {
                break;
            } else {
                identical += 1;
            }
        }
        identical
    }

    /// Updates the Levenshtein matrix based on the new query string.
    /// This method should be called before calculating similarity if the query has changed.
    ///
    /// # Arguments
    ///
    /// * `new_query` - A slice of the new query string.
    fn update(&mut self, new_query: &str) {
        let query_similarity = self.query_similarity(new_query);

        if new_query.len() > self.query.len() {
            for _ in 0..(new_query.len() - self.query.len()) {
                let row = vec![0; self.data.len() + 1];
                self.matrix.push(row);
            }
        } else {
            for _ in 0..(self.query.len() - new_query.len()) {
                self.matrix.pop();
            }
        }

        self.query = new_query.to_string();

        let b = &self.data;
        let len_a = self.query.len();
        let len_b = self.data.len();

        self.matrix[len_a][0] = len_a;

        for i in (query_similarity + 1)..=len_a {
            for j in 1..=len_b {
                let cost = if self.query.chars().nth(i - 1) == b.chars().nth(j - 1) {
                    0
                } else {
                    1
                };

                self.matrix[i][j] = std::cmp::min(
                    self.matrix[i - 1][j] + 1,
                    std::cmp::min(self.matrix[i][j - 1] + 1, self.matrix[i - 1][j - 1] + cost),
                );
            }
        }
    }

    /// Calculates the similarity ratio between the stored data string and the new query string
    /// after updating the internal state with the new query.
    ///
    /// # Arguments
    ///
    /// * `new_query` - A slice of the new query string to compare.
    ///
    /// # Returns
    ///
    /// A `f64` representing the similarity ratio (0.0 meaning no similarity and 1.0 meaning identical).
    pub fn similarity(&mut self, new_query: &str) -> f64 {
        self.update(new_query);
        let distance = self.matrix[self.query.len()][self.data.len()];
        let max_distance = self.query.len().max(self.data.len());
        if max_distance == 0 {
            0.
        } else {
            (max_distance - distance) as f64 / max_distance as f64
        }
    }

    /// Calculates a weighted similarity ratio, which considers the length and type of edit
    /// operations required to convert the query into the data string.
    ///
    /// # Arguments
    ///
    /// * `new_query` - A slice of the new query string to compare.
    ///
    /// # Returns
    ///
    /// A `f64` representing the weighted similarity ratio.
    pub fn weighted_similarity(&mut self, new_query: &str) -> f64 {
        self.update(new_query);
        weighted_edit_similarity(&self.matrix, &self.query, &self.data)
    }
}
