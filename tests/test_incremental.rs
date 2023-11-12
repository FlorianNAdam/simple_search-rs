fn print_matrix(matrix: &Vec<Vec<usize>>) {
    for row in matrix {
        println!("{:?}", row);
    }
}

#[cfg(test)]
mod tests {
    use crate::print_matrix;
    use rand::distributions::{Alphanumeric, DistString};
    use rand::prelude::*;
    use simple_search::levenshtein::base::{levenshtein_matrix, weighted_levenshtein_similarity};
    use simple_search::levenshtein::incremental::IncrementalLevenshtein;
    use simple_search::search_engine::SearchEngine;
    use std::collections::HashMap;

    #[test]
    fn test_incremental() {
        let mut rng = StdRng::seed_from_u64(42);

        let num_entries = rng.gen_range(1000..=2000);
        let data: Vec<_> = (0..num_entries)
            .map(|_| {
                let str_len = rng.gen_range(10..=100);
                Alphanumeric.sample_string(&mut rng, str_len)
            })
            .collect();

        let regular = SearchEngine::new()
            .with_values(data.clone())
            .with(|v, q| weighted_levenshtein_similarity(q, v));

        let mut incremental = SearchEngine::new().with_values(data.clone()).with_state(
            |v| IncrementalLevenshtein::new("", v),
            |s, _, q| s.weighted_similarity(q),
        );

        let mut query = Alphanumeric.sample_string(&mut rng, 16);

        for _ in 0..20 {
            let addition = Alphanumeric.sample_string(&mut rng, 1);

            let index = rng.gen_range(0..=query.len());
            query.insert_str(index, &addition);

            let mut regular_similarities = HashMap::new();
            regular.similarities(&query).into_iter().for_each(|(v, s)| {
                regular_similarities.insert(v.to_string(), s);
            });

            let mut incremental_similarities = HashMap::new();
            incremental
                .similarities(&query)
                .into_iter()
                .for_each(|(v, s)| {
                    incremental_similarities.insert(v.to_string(), s);
                });

            assert_eq!(regular_similarities.len(), incremental_similarities.len());

            for key in regular_similarities.keys() {
                let regular_similarity = regular_similarities.get(key).unwrap().clone();
                let incremental_similarity = incremental_similarities.get(key).unwrap().clone();

                if regular_similarity != incremental_similarity {
                    println!("Key: {}", key);
                    println!("Query: {}", query);
                    println!(
                        "Granular: {}, Incremental: {}",
                        regular_similarity, incremental_similarity
                    );

                    let regular_matrix = levenshtein_matrix(&query, key);
                    let values = incremental.get_values_with_state();

                    let incremental_matrix = values
                        .into_iter()
                        .find(|(_, v)| v == key)
                        .unwrap()
                        .0
                        .clone();
                    let incremental_matrix = incremental_matrix.0.matrix();

                    if incremental_matrix != &regular_matrix {
                        println!("Regular matrix and incremental matrix do not match");

                        println!("Regular Matrix:");
                        print_matrix(&regular_matrix);
                        println!("Incremental Matrix:");
                        print_matrix(&incremental_matrix);
                    } else {
                        panic!("Regular and incremental similarities do not match");
                    }
                }
            }
        }
    }
}
