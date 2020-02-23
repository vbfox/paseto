use criterion::{Criterion};

mod local;

pub fn benches(c: &mut Criterion) {
    local::benches(c);
}