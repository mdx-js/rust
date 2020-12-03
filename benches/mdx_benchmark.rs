use criterion::{black_box, criterion_group, criterion_main, Criterion};
use mdx::parse;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("parse", |b| b.iter(|| parse(black_box("# Some"))));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);