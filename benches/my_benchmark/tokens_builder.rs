use std::iter;
use std::str;
use chrono::prelude::*;
use serde_json::json;
use criterion::{black_box, criterion_group, criterion_main, Criterion, Bencher, Throughput, BenchmarkId};
use paseto::tokens::builder::PasetoBuilder;

fn bench_construct_local(b: &mut Bencher, claim: &str, footer: &str) {
  b.iter(|| {
    PasetoBuilder::new()
      .set_encryption_key(Vec::from("YELLOW SUBMARINE, BLACK WIZARDRY".as_bytes()))
      .set_issued_at(None)
      .set_expiration(Utc::now())
      .set_issuer(String::from("issuer"))
      .set_audience(String::from("audience"))
      .set_jti(String::from("jti"))
      .set_not_before(Utc::now())
      .set_subject(String::from("test"))
      .set_claim(String::from("claim"), json!(claim))
      .set_footer(String::from(footer))
      .build()
  });
}

pub fn benches(c: &mut Criterion) {
  static KB: usize = 1024;

  let mut group = c.benchmark_group("token::builder::local");
  for size in [1, 1 * KB, 16 * KB].iter() {
      group.throughput(Throughput::Bytes(*size as u64));
      group.bench_with_input(BenchmarkId::from_parameter(size), size, |b, &size| {
        let bytes = iter::repeat(20u8).take(size).collect::<Vec<_>>();
        let s = str::from_utf8(&bytes).unwrap();

        bench_construct_local(b, s, s);
      });
  }
  group.finish();
}