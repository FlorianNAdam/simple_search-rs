use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use rand::distributions::{Alphanumeric, DistString};
use rand::prelude::StdRng;
use rand::{Rng, SeedableRng};
use simple_search::levenshtein::base::weighted_levenshtein_similarity;
use simple_search::levenshtein::incremental::IncrementalLevenshtein;
use simple_search::search_engine::SearchEngine;
use std::collections::HashMap;

fn bench_erasure_overhead(c: &mut Criterion) {
    let mut group = c.benchmark_group("BenchErasureOverhead");

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
        .with(|v, q| weighted_levenshtein_similarity(v, q));

    let erased = SearchEngine::new()
        .with_values(data.clone())
        .with(|v, q| weighted_levenshtein_similarity(v, q))
        .erase_type();

    let mut query = Alphanumeric.sample_string(&mut rng, 16);

    for i in 0..20 {
        let addition = Alphanumeric.sample_string(&mut rng, 1);

        let index = rng.gen_range(0..=query.len());
        query.insert_str(index, &addition);

        // regular
        group.bench_function(BenchmarkId::new("Regular", i), |b| {
            b.iter(|| {
                black_box(regular.similarities(&query));
            })
        });

        // erased
        group.bench_function(BenchmarkId::new("Erased", i), |b| {
            b.iter(|| {
                black_box(erased.similarities(&query));
            })
        });

        let granular_similarity = regular.similarities(&query);
        let erased_similarity = erased.similarities(&query);

        assert_eq!(granular_similarity, erased_similarity);
    }
    group.finish();
}

fn bench_incremental(c: &mut Criterion) {
    let mut group = c.benchmark_group("CompareIncremental");

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

    let erased = SearchEngine::new()
        .with_values(data.clone())
        .with(|v, q| weighted_levenshtein_similarity(q, v))
        .erase_type();

    let mut incremental = SearchEngine::new().with_values(data.clone()).with_state(
        |v| IncrementalLevenshtein::new("", v),
        |s, _, q| s.weighted_similarity(q),
    );

    let mut incremental_erased = SearchEngine::new()
        .with_values(data.clone())
        .with_state(
            |v| IncrementalLevenshtein::new("", v),
            |s, _, q| s.weighted_similarity(q),
        )
        .erase_type_cloneable();

    let mut query = Alphanumeric.sample_string(&mut rng, 16);

    for i in 0..20 {
        let addition = Alphanumeric.sample_string(&mut rng, 1);

        let index = rng.gen_range(0..=query.len());
        query.insert_str(index, &addition);

        // regular
        group.bench_function(BenchmarkId::new("Regular", i), |b| {
            b.iter(|| {
                black_box(regular.similarities(&query));
            })
        });

        // erased
        group.bench_function(BenchmarkId::new("Erased", i), |b| {
            b.iter(|| {
                black_box(erased.similarities(&query));
            })
        });

        // incremental
        group.bench_function(BenchmarkId::new("Incremental", i), |b| {
            b.iter_with_setup(
                || incremental.clone(),
                |mut incremental| {
                    black_box(incremental.similarities(&query));
                },
            )
        });

        // incremental_erased
        group.bench_function(BenchmarkId::new("IncrementalErased", i), |b| {
            b.iter_with_setup(
                || incremental_erased.clone(),
                |mut incremental_erased| {
                    black_box(incremental_erased.similarities(&query));
                },
            )
        });

        let mut granular_similarities = HashMap::new();
        regular.similarities(&query).into_iter().for_each(|(v, s)| {
            granular_similarities.insert(v, s);
        });
        let mut erased_similarities = HashMap::new();
        erased.similarities(&query).into_iter().for_each(|(v, s)| {
            erased_similarities.insert(v, s);
        });
        let mut incremental_similarities = HashMap::new();
        incremental
            .similarities(&query)
            .into_iter()
            .for_each(|(v, s)| {
                incremental_similarities.insert(v, s);
            });
        let mut incremental_erased_similarities = HashMap::new();
        incremental_erased
            .similarities(&query)
            .into_iter()
            .for_each(|(v, s)| {
                incremental_erased_similarities.insert(v, s);
            });

        assert_eq!(granular_similarities, erased_similarities);
        assert_eq!(granular_similarities, incremental_similarities);
        assert_eq!(granular_similarities, incremental_erased_similarities);
    }
    group.finish();
}

criterion_group!(benches, bench_erasure_overhead, bench_incremental);
criterion_main!(benches);
