//! This module defines functions and data structures for calculating the Levenshtein distance
//! and similarity between two strings

/// Computes the Levenshtein distance between two strings.
///
/// # Arguments
///
/// * `a` - The first string to compare.
/// * `b` - The second string to compare.
///
/// # Returns
///
/// Returns the Levenshtein distance as a `usize`.
pub fn levenshtein_distance(a: &str, b: &str) -> usize {
    let len_a = a.len();
    let len_b = b.len();
    let matrix = levenshtein_matrix(a, b);
    matrix[len_a][len_b]
}

/// Computes the similarity ratio based on the Levenshtein distance between two strings.
///
/// # Arguments
///
/// * `a` - The first string to compare.
/// * `b` - The second string to compare.
///
/// # Returns
///
/// Returns a `f64` representing the similarity ratio, where 1.0 is identical and 0.0 is completely dissimilar.
pub fn levenshtein_similarity(a: &str, b: &str) -> f64 {
    let distance = levenshtein_distance(a, b);
    let max_distance = a.len().max(b.len());
    if max_distance == 0 {
        0.
    } else {
        (max_distance - distance) as f64 / max_distance as f64
    }
}

pub fn weighted_levenshtein_similarity(a: &str, b: &str) -> f64 {
    let matrix = levenshtein_matrix(a, b);
    weighted_edit_similarity(&matrix, a, b)
}

/// Generates a matrix used to compute the Levenshtein distance between two strings.
///
/// # Arguments
///
/// * `a` - The first string to compare.
/// * `b` - The second string to compare.
///
/// # Returns
///
/// Returns a matrix (`Vec<Vec<usize>>`) representing the costs of edits required to change the first string into the second.
pub(crate) fn levenshtein_matrix(a: &str, b: &str) -> Vec<Vec<usize>> {
    let len_a = a.len();
    let len_b = b.len();

    // Create a matrix.
    let mut matrix = vec![vec![0; len_b + 1]; len_a + 1];

    // Initialize the matrix.
    for i in 0..=len_a {
        matrix[i][0] = i;
    }
    for j in 0..=len_b {
        matrix[0][j] = j;
    }

    // Compute the Levenshtein distance.
    for i in 1..=len_a {
        for j in 1..=len_b {
            let cost = if a.chars().nth(i - 1) == b.chars().nth(j - 1) {
                0
            } else {
                1
            };

            matrix[i][j] = std::cmp::min(
                matrix[i - 1][j] + 1,
                std::cmp::min(matrix[i][j - 1] + 1, matrix[i - 1][j - 1] + cost),
            );
        }
    }

    matrix
}

/// Represents an edit operation in the Levenshtein distance algorithm.
#[derive(Debug)]
pub(crate) enum EditOperation {
    Insert(usize),
    Delete(usize),
    Substitute(usize, usize),
    None(usize),
}

/// Calculates the edit operations required to transform the original string into the target string
/// based on the Levenshtein matrix.
///
/// # Arguments
///
/// * `matrix` - The Levenshtein matrix representing the edit distances.
/// * `original` - The original string.
/// * `target` - The target string to transform into.
///
/// # Returns
///
/// Returns a vector of `EditOperation` which are the steps needed to convert the original string into the target string.
pub(crate) fn edit_operations(
    matrix: &Vec<Vec<usize>>,
    original: &str,
    target: &str,
) -> Vec<EditOperation> {
    let mut operations = Vec::new();
    let mut i = original.len();
    let mut j = target.len();

    while i > 0 && j > 0 {
        let current = matrix[i][j];
        let deletion = matrix[i - 1][j] + 1;
        let insertion = matrix[i][j - 1] + 1;
        let substitution = matrix[i - 1][j - 1]
            + if original.chars().nth(i - 1) == target.chars().nth(j - 1) {
                0
            } else {
                1
            };

        if current == substitution {
            if original.chars().nth(i - 1) != target.chars().nth(j - 1) {
                operations.push(EditOperation::Substitute(1, 1)); // Substituting one char for another
            }
            i -= 1;
            j -= 1;
        } else if current == deletion {
            // Count the number of deletions.
            let mut del_count = 0;
            while i > 0 && matrix[i][j] == matrix[i - 1][j] + 1 {
                del_count += 1;
                i -= 1;
            }
            operations.push(EditOperation::Delete(del_count));
        } else if current == insertion {
            // Count the number of insertions.
            let mut ins_count = 0;
            while j > 0 && matrix[i][j] == matrix[i][j - 1] + 1 {
                ins_count += 1;
                j -= 1;
            }
            operations.push(EditOperation::Insert(ins_count));
        }
    }

    // Handle remaining deletions.
    if i > 0 {
        operations.push(EditOperation::Delete(i));
    }

    // Handle remaining insertions.
    if j > 0 {
        operations.push(EditOperation::Insert(j));
    }

    operations.reverse(); // Reverse to get the correct order of operations.
    operations
}

pub(crate) fn edit_operations_2(
    matrix: &Vec<Vec<usize>>,
    original: &str,
    target: &str,
) -> Vec<EditOperation> {
    let mut operations = Vec::new();
    let mut i = original.len();
    let mut j = target.len();

    while i > 0 && j > 0 {
        let current = matrix[i][j];
        let deletion = matrix[i - 1][j] + 1;
        let insertion = matrix[i][j - 1] + 1;
        let substitution = matrix[i - 1][j - 1]
            + if original.chars().nth(i - 1) == target.chars().nth(j - 1) {
                0
            } else {
                1
            };

        // No change needed, move diagonally without any operation
        if original.chars().nth(i - 1) == target.chars().nth(j - 1) {
            i -= 1;
            j -= 1;
            continue;
        }

        if current == substitution {
            // Substituting one char for another
            operations.push(EditOperation::Substitute(1, 1));
            i -= 1;
            j -= 1;
        } else if current == deletion {
            // Count the number of deletions
            let mut del_count = 0;
            while i > 0 && matrix[i][j] == matrix[i - 1][j] + 1 {
                del_count += 1;
                i -= 1;
            }
            operations.push(EditOperation::Delete(del_count));
        } else if current == insertion {
            // Count the number of insertions
            let mut ins_count = 0;
            while j > 0 && matrix[i][j] == matrix[i][j - 1] + 1 {
                ins_count += 1;
                j -= 1;
            }
            operations.push(EditOperation::Insert(ins_count));
        } else {
            // If the cost is the same as the diagonal, it means no operation needed.
            let mut no_change_count = 0;
            while i > 0 && j > 0 && matrix[i][j] == matrix[i - 1][j - 1] {
                i -= 1;
                j -= 1;
                no_change_count += 1;
            }
            operations.push(EditOperation::None(no_change_count));
        }
    }

    // Handle remaining deletions
    if i > 0 {
        operations.push(EditOperation::Delete(i));
    }

    // Handle remaining insertions
    if j > 0 {
        operations.push(EditOperation::Insert(j));
    }

    operations.reverse(); // Reverse to get the correct order of operations
    operations
}

pub(crate) fn weighted_edit_similarity(matrix: &Vec<Vec<usize>>, a: &str, b: &str) -> f64 {
    let ops = edit_operations(matrix, a, b);

    let mut distance = 0.;

    for op in ops {
        match op {
            EditOperation::Insert(len) => distance += (len as f64).ln_1p(),
            EditOperation::Delete(len) => distance += (len as f64).ln_1p(),
            EditOperation::Substitute(len_a, len_b) => {
                distance += (len_a as f64).ln_1p();
                distance += (len_b as f64).ln_1p();
            }
            EditOperation::None(_) => {}
        }
    }

    let max_distance = a.len().max(b.len());
    if max_distance == 0 {
        0.
    } else {
        (max_distance as f64 - distance) / max_distance as f64
    }
}

pub(crate) fn weighted_edit_similarity_2(matrix: &Vec<Vec<usize>>, a: &str, b: &str) -> f64 {
    let ops = edit_operations(matrix, a, b);

    let mut distance = 0.;

    for op in ops {
        match op {
            EditOperation::Insert(len) => distance += (len as f64).ln_1p(),
            EditOperation::Delete(len) => distance += (len as f64).ln_1p(),
            EditOperation::Substitute(len_a, len_b) => {
                distance += (len_a as f64).ln_1p();
                distance += (len_b as f64).ln_1p();
            }
            EditOperation::None(_) => {}
        }
    }

    let max_distance = a.len().max(b.len());
    if max_distance == 0 {
        0.
    } else {
        (max_distance as f64 - distance) / max_distance as f64
    }
}
