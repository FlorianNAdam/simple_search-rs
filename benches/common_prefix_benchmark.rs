use criterion::{black_box, criterion_group, criterion_main, BenchmarkId, Criterion};
use rand::distributions::{Alphanumeric, DistString};
use rand::prelude::StdRng;
use rand::{Rng, SeedableRng};
use simple_search::levenshtein::base::common_prefix;

fn bench_common_prefix(c: &mut Criterion) {
    let mut group = c.benchmark_group("CommonPrefix");

    let mut rng = StdRng::seed_from_u64(42);

    for i in 0..20 {
        let len_a = rng.gen_range(5000000..20000000);
        let a = Alphanumeric.sample_string(&mut rng, len_a);

        let len_b = rng.gen_range(5000000..20000000);
        let b = Alphanumeric.sample_string(&mut rng, len_b);

        group.bench_function(BenchmarkId::new("Simple", i), |bencher| {
            bencher.iter(|| {
                black_box(common_prefix(&a, &b));
            })
        });
    }
    group.finish();
}

criterion_group!(benches, bench_common_prefix);
criterion_main!(benches);
