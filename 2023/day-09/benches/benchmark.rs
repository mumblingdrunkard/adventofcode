use criterion::{criterion_group, criterion_main, Criterion};
use day_09::solve;

pub fn benchmark(c: &mut Criterion) {
    let lines = std::fs::read_to_string("input").unwrap();
    c.bench_function("solve lines", |b| {
        b.iter(|| solve(std::hint::black_box(lines.lines())))
    });
}

criterion_group!(benches, benchmark);
criterion_main!(benches);
