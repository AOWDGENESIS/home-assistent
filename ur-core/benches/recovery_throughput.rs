use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn signature_search_benchmark(c: &mut Criterion) {
    let data = vec![0_u8; 1024 * 1024];
    let signature = [0xFF, 0xD8, 0xFF];
    c.bench_function("signature_search_1mb", |b| {
        b.iter(|| black_box(ur_core::recovery::find_signature(&data, &signature)))
    });
}

criterion_group!(benches, signature_search_benchmark);
criterion_main!(benches);
