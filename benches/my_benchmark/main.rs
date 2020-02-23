use criterion::{criterion_group, criterion_main, Criterion};

mod tokens_builder;
mod utils;

pub fn criterion_benchmark(c: &mut Criterion) {
  tokens_builder::benches(c);
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
