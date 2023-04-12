use aoc::*;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn d01(c: &mut Criterion) {
    use d01::{part2::solution, INPUT};
    c.bench_function("d01", |b| b.iter(|| solution(black_box(INPUT))));
}

fn d02(c: &mut Criterion) {
    use d02::{part2::solution, INPUT};
    c.bench_function("d02", |b| b.iter(|| solution(black_box(INPUT))));
}

fn d03(c: &mut Criterion) {
    use d03::{part2::solution, INPUT};
    c.bench_function("d03", |b| b.iter(|| solution(black_box(INPUT))));
}

fn d04(c: &mut Criterion) {
    use d04::{part2::solution, INPUT};
    c.bench_function("d04", |b| b.iter(|| solution(black_box(INPUT))));
}

fn d05(c: &mut Criterion) {
    use d05::{part2::solution, INPUT};
    c.bench_function("d05", |b| b.iter(|| solution(black_box(INPUT))));
}

fn d06(c: &mut Criterion) {
    use d06::{part2::solution, INPUT};
    c.bench_function("d06", |b| b.iter(|| solution(black_box(INPUT))));
}

fn d07(c: &mut Criterion) {
    use d07::{part2::solution, INPUT};
    c.bench_function("d07", |b| b.iter(|| solution(black_box(INPUT))));
}

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

fn d13(c: &mut Criterion) {
    use d13::{part2::solution, INPUT};
    c.bench_function("d13", |b| b.iter(|| solution(black_box(INPUT))));
}

fn d14(c: &mut Criterion) {
    use d14::{part2::solution, INPUT};
    c.bench_function("d14", |b| b.iter(|| solution(black_box(INPUT))));
}

fn d15(c: &mut Criterion) {
    use d15::{part2::solution, INPUT};
    c.bench_function("d15", |b| b.iter(|| solution(black_box(INPUT), 4000000)));
}

criterion_group!(
    benches, d01, d02, d03, d04, d05, d06, d07, d08, d09, d10, d11, d12, d13, d14, d15
);
criterion_main!(benches);
