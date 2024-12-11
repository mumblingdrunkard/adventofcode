use criterion::{criterion_group, criterion_main, Criterion};
use day_16::solve;

pub fn benchmark(c: &mut Criterion) {
    c.bench_function("solve lines", |b| {
        b.iter(|| {
            solve(std::hint::black_box(
                std::fs::read_to_string("input").unwrap().lines(),
            ))
        })
    });
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
