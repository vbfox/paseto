use criterion::{criterion_group, criterion_main, Criterion};

mod tokens;
mod utils;
mod pae;

pub fn criterion_benchmark(c: &mut Criterion) {
  tokens::benches(c);
  pae::benches(c);
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
