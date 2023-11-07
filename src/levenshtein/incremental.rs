use crate::levenshtein::base::{edit_operations, levenshtein_matrix, EditOperation};

#[derive(Clone)]
pub struct IncrementalLevenshtein {
    query: String,
    data: String,
    matrix: Vec<Vec<usize>>,
}

impl IncrementalLevenshtein {
    pub fn new(query: &str, data: &str) -> Self {
        Self {
            query: query.to_string(),
            data: data.to_string(),
            matrix: levenshtein_matrix(query, data),
        }
    }

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

    pub fn weighted_similarity(&mut self, new_query: &str) -> f64 {
        self.update(new_query);

        let ops = edit_operations(&self.matrix, &self.query, &self.data);

        let mut distance = 0.;

        for op in ops {
            match op {
                EditOperation::Insert(len) => distance += (len as f64).ln_1p(),
                EditOperation::Delete(len) => distance += (len as f64).ln_1p(),
                EditOperation::Substitute(len_a, len_b) => {
                    distance += (len_a as f64).ln_1p();
                    distance += (len_b as f64).ln_1p();
                }
            }
        }

        let max_distance = self.query.len().max(self.data.len());
        if max_distance == 0 {
            0.
        } else {
            (max_distance as f64 - distance) / max_distance as f64
        }
    }
}
