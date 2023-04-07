use aoc::*;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn d8(c: &mut Criterion) {
    use d8::{part2::solution, INPUT};
    c.bench_function("d8", |b| b.iter(|| solution(black_box(INPUT))));
}

fn d9(c: &mut Criterion) {
    use d9::{part2::solution, INPUT};
    c.bench_function("d9", |b| b.iter(|| solution(black_box(INPUT))));
}

criterion_group!(benches, d8, d9);
criterion_main!(benches);
