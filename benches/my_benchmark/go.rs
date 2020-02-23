use criterion::{black_box, Criterion, Bencher};
use paseto::tokens::{
  validate_local_token,
};

/// Port of https://github.com/o1egl/paseto/blob/master/benchmarks_test.go
/// Benchmark_V2_JSONToken_Decrypt function
fn bench_go_decrypt(b: &mut Bencher) {
  let token = "v2.local.Ydp5u4gIRRR6u7Nvdb64qJs1W2wKSHNNEmi0LCnuZ9s-j74qrYu77tMzbUZvILPQE9Pl3OxPo246BUqQQ38YbZtQ2Stw8SJbvSbwF7npAjMkTx4leorq-bez8i9jLuyv7dHy8F4JaN8XxoNSpQdKI4Gn567sY-YxvBDTcEtM-VwRfe6vXHk_QG6pfil0hemk3zOAHPq0GxCA_uQnx6ggYN4mP_rqKdYV2P6Myf9nZmc-sw1hHCMSZegx6OH1nrKzvzMA9Y2ZO_tsg8IACz_wG2Mk.Zm9vdGVy";
  b.iter(|| {
    let symmetric_key = Vec::from("YELLOW SUBMARINE, BLACK WIZARDRY".as_bytes());
    validate_local_token(black_box(&token), black_box(Some("footer")), black_box(symmetric_key))
      .expect("Couldn't verify v2 local paseto")
  })
}

pub fn benches(c: &mut Criterion) {
  c.bench_function("go::Benchmark_V2_JSONToken_Decrypt", &bench_go_decrypt);
}