use aoc::*;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn d08(c: &mut Criterion) {
    use d08::{part2::solution, INPUT};
    c.bench_function("d08", |b| b.iter(|| solution(black_box(INPUT))));
}

fn d09(c: &mut Criterion) {
    use d09::{part2::solution, INPUT};
    c.bench_function("d09", |b| b.iter(|| solution(black_box(INPUT))));
}

criterion_group!(benches, d08, d09);
criterion_main!(benches);
