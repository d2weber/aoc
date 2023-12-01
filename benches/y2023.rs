use aoc::y2023::*;
use criterion::{black_box, criterion_group, Criterion};

fn d01(c: &mut Criterion) {
    use d01::{part2::solution, INPUT};
    c.bench_function("y2023::d01", |b| b.iter(|| solution(black_box(INPUT))));
}

criterion_group!(benches, d01);
