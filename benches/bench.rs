
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use sha_x_n::collider;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("Collider 8", |b| b.iter(|| collider(black_box(8))));
    c.bench_function("Collider 16", |b| b.iter(|| collider(black_box(16))));
    c.bench_function("Collider 24", |b| b.iter(|| collider(black_box(24))));
    c.bench_function("Collider 32", |b| b.iter(|| collider(black_box(32))));
    c.bench_function("Collider 40", |b| b.iter(|| collider(black_box(40))));
    c.bench_function("Collider 48", |b| b.iter(|| collider(black_box(48))));
}

criterion_group!{
    name = benches;
    // This can be any expression that returns a `Criterion` object.
    config = Criterion::default().significance_level(0.1).sample_size(10);
    targets = criterion_benchmark
}
criterion_main!(benches);