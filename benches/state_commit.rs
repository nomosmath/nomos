use criterion::{criterion_group, criterion_main, Criterion};
use nomos_core::state::{StateTrie, ProofRecord};

fn bench_insert(c: &mut Criterion) {
    c.bench_function("state_trie_insert_1000", |b| {
        b.iter(|| {
            let mut trie = StateTrie::new();
            for i in 0..1000 {
                trie.insert(ProofRecord {
                    proof_id: format!("proof-{}", i),
                    submitter: "bench".into(),
                    verified: true,
                    epoch: 1,
                    hash: [0u8; 32],
                });
            }
        });
    });
}

criterion_group!(benches, bench_insert);
criterion_main!(benches);
