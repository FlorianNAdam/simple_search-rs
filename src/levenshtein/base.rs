pub fn levenshtein_distance(a: &str, b: &str) -> usize {
    let len_a = a.len();
    let len_b = b.len();
    let matrix = levenshtein_matrix(a, b);
    matrix[len_a][len_b]
}

pub fn levenshtein_similarity(a: &str, b: &str) -> f64 {
    let distance = levenshtein_distance(a, b);
    let max_distance = a.len().max(b.len());
    if max_distance == 0 {
        0.
    } else {
        (max_distance - distance) as f64 / max_distance as f64
    }
}

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

#[derive(Debug)]
pub(crate) enum EditOperation {
    Insert(usize),
    Delete(usize),
    Substitute(usize, usize),
}

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
