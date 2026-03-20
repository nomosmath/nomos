use criterion::{criterion_group, criterion_main, Criterion};
use nomos_core::consensus::ValidatorSet;

fn bench_quorum(c: &mut Criterion) {
    c.bench_function("quorum_100_validators", |b| {
        b.iter(|| {
            let mut set = ValidatorSet::new(0.67);
            for i in 0..100 {
                set.register(format!("v-{}", i), 1000);
            }
            assert_eq!(set.active_count(), 100);
        });
    });
}

criterion_group!(benches, bench_quorum);
criterion_main!(benches);
