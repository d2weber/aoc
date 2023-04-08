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

fn d10(c: &mut Criterion) {
    use d10::{part2::solution, INPUT};
    c.bench_function("d10", |b| b.iter(|| solution(black_box(INPUT))));
}

fn d11(c: &mut Criterion) {
    use d11::{part2::solution, INPUT};
    c.bench_function("d11", |b| b.iter(|| solution(black_box(INPUT))));
}

fn d12(c: &mut Criterion) {
    use d12::{part1, part2, INPUT};
    c.bench_function("d12:part1", |b| {
        b.iter(|| part1::solution(black_box(INPUT)))
    });
    c.bench_function("d12:part2", |b| {
        b.iter(|| part2::solution(black_box(INPUT)))
    });
}

criterion_group!(benches, d08, d09, d10, d11, d12);
criterion_main!(benches);
