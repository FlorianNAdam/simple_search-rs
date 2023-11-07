use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use rand::distributions::{Alphanumeric, DistString};
use rand::prelude::StdRng;
use rand::{Rng, SeedableRng};
use simple_similarity::levenshtein::{levenshtein_distance, IncrementalLevenshtein};

fn bench_levenshtein_random_insert(c: &mut Criterion) {
    let mut group = c.benchmark_group("Levenshtein");

    let mut rng = StdRng::seed_from_u64(42);

    let data = Alphanumeric.sample_string(&mut rng, 160);
    let mut query = Alphanumeric.sample_string(&mut rng, 16);

    let mut fast_lv = IncrementalLevenshtein::new(&query, &data);

    for i in 0..100 {
        let addition = Alphanumeric.sample_string(&mut rng, 1);

        let index = rng.gen_range(0..=query.len());
        query.insert_str(index, &addition);

        group.bench_function(BenchmarkId::new("Incremental", i), |b| {
            b.iter_with_setup(
                || fast_lv.clone(),
                |mut prepared_lv| {
                    prepared_lv.similarity(&query);
                },
            )
        });
        group.bench_function(BenchmarkId::new("Naive", i), |b| {
            b.iter(|| levenshtein_distance(&query, &data))
        });

        let slow_lv_sim = levenshtein_distance(&query, &data);
        let fast_lv_sim = fast_lv.similarity(&query);
        assert_eq!(slow_lv_sim, fast_lv_sim);
    }
    group.finish();
}

fn bench_levenshtein_random_append(c: &mut Criterion) {
    let mut group = c.benchmark_group("Levenshtein");

    let mut rng = StdRng::seed_from_u64(17);

    let data = Alphanumeric.sample_string(&mut rng, 160);
    let mut query = Alphanumeric.sample_string(&mut rng, 16);

    let mut fast_lv = IncrementalLevenshtein::new(&query, &data);

    for i in 0..10 {
        let addition = Alphanumeric.sample_string(&mut rng, 1);

        query.push_str(&addition);

        group.bench_function(BenchmarkId::new("Incremental", i), |b| {
            b.iter_with_setup(
                || fast_lv.clone(),
                |mut prepared_lv| {
                    black_box(prepared_lv.similarity(&query));
                },
            )
        });
        group.bench_function(BenchmarkId::new("Naive", i), |b| {
            b.iter(|| black_box(levenshtein_distance(&query, &data)))
        });

        let slow_lv_sim = levenshtein_distance(&query, &data);
        let fast_lv_sim = fast_lv.similarity(&query);
        assert_eq!(slow_lv_sim, fast_lv_sim);
    }
    group.finish();
}

criterion_group!(
    benches,
    bench_levenshtein_random_insert,
    bench_levenshtein_random_append
);
criterion_main!(benches);
