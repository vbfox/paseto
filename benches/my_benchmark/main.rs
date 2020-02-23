use criterion::{criterion_group, criterion_main, Criterion};

mod tokens;
mod utils;
mod pae;
mod v2;
mod go;

pub fn criterion_benchmark(c: &mut Criterion) {
  tokens::benches(c);
  pae::benches(c);
  v2::benches(c);
  go::benches(c);
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);

/*
G:\Code\_ext\paseto-go>go test -bench . -benchmem
goos: windows
goarch: amd64
pkg: github.com/o1egl/paseto/v2
Benchmark_V2_JSONToken_Encrypt-8          142813              8417 ns/op            4185 B/op         59 allocs/op
Benchmark_V2_JSONToken_Decrypt-8          149946              8090 ns/op            2048 B/op         63 allocs/op
Benchmark_V2_JSONToken_Sign-8              21466             55764 ns/op            4425 B/op         60 allocs/op
Benchmark_V2_JSONToken_Verify-8             8571            133357 ns/op            2528 B/op         64 allocs/op
Benchmark_V2_String_Encrypt-8             665797              1729 ns/op            1176 B/op         23 allocs/op
Benchmark_V2_String_Decrypt-8            1000000              1060 ns/op             568 B/op         18 allocs/op
Benchmark_V2_String_Sign-8                 24946             48025 ns/op            1144 B/op         23 allocs/op
Benchmark_V2_String_Verify-8                9976            125103 ns/op             744 B/op         18 allocs/op
PASS
ok      github.com/o1egl/paseto/v2      10.739s

*/