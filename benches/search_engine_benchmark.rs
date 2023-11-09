use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use rand::distributions::{Alphanumeric, DistString};
use rand::prelude::StdRng;
use rand::{Rng, SeedableRng};
use simple_search::levenshtein::base::{levenshtein_similarity, weighted_levenshtein_similarity};
use simple_search::{granular, stateful, stateless};

fn bench_search_engine_overhead(c: &mut Criterion) {
    let mut group = c.benchmark_group("SearchEngineOverhead");

    let mut rng = StdRng::seed_from_u64(42);

    let num_entries = rng.gen_range(100..=200);
    let data: Vec<_> = (0..num_entries)
        .map(|_| {
            let str_len = rng.gen_range(10..=100);
            Alphanumeric.sample_string(&mut rng, str_len)
        })
        .collect();

    let granular = granular::search_engine::SearchEngine::new()
        .with_values(data.clone())
        .with(|v, q: &&str| weighted_levenshtein_similarity(v, q));
    // let stateless = stateless::search_engine::SearchEngine::new()
    //     .with_values(data.clone())
    //     .with_key_fn(|v, q: &&str| weighted_levenshtein_similarity(v, q));
    // let mut stateful = stateful::search_engine::SearchEngine::new::<()>()
    //     .with_values(data.clone())
    //     .with_key_fn(|_, v, q: &&str| weighted_levenshtein_similarity(v, q));

    let data = Alphanumeric.sample_string(&mut rng, 160);
    let mut query = Alphanumeric.sample_string(&mut rng, 16);

    for i in 0..20 {
        let addition = Alphanumeric.sample_string(&mut rng, 1);

        let index = rng.gen_range(0..=query.len());
        query.insert_str(index, &addition);

        // granular
        group.bench_function(BenchmarkId::new("Granular", i), |b| {
            b.iter(|| {
                let query = query.clone();
                granular.similarities(&(query.as_str()));
            })
        });

        // // stateless
        // group.bench_function(BenchmarkId::new("Stateless", i), |b| {
        //     b.iter(|| stateless.similarities(&query))
        // });
        //
        // // stateful
        // group.bench_function(BenchmarkId::new("Stateful", i), |b| {
        //     b.iter_with_setup(
        //         || stateful.clone(),
        //         |mut stateful| {
        //             stateful.similarities(&query);
        //         },
        //     )
        // });

        // let granular_similarity = granular.similarities(&query);
        // let stateless_similarity = stateless.similarities(&query);
        // let stateful_similarity = stateful.similarities(&query);

        // assert_eq!(granular_similarity, stateless_similarity);
        // assert_eq!(granular_similarity, stateful_similarity);
    }
    group.finish();
}

criterion_group!(benches, bench_search_engine_overhead);
criterion_main!(benches);
