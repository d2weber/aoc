use aoc::y2022::*;
use criterion::{black_box, criterion_group, Criterion};

fn d01(c: &mut Criterion) {
    use d01::{part2::solution, INPUT};
    c.bench_function("y2022::d01", |b| b.iter(|| solution(black_box(INPUT))));
}

fn d02(c: &mut Criterion) {
    use d02::{part2::solution, INPUT};
    c.bench_function("y2022::d02", |b| b.iter(|| solution(black_box(INPUT))));
}

fn d03(c: &mut Criterion) {
    use d03::{part2::solution, INPUT};
    c.bench_function("y2022::d03", |b| b.iter(|| solution(black_box(INPUT))));
}

fn d04(c: &mut Criterion) {
    use d04::{part2::solution, INPUT};
    c.bench_function("y2022::d04", |b| b.iter(|| solution(black_box(INPUT))));
}

fn d05(c: &mut Criterion) {
    use d05::{part2::solution, INPUT};
    c.bench_function("y2022::d05", |b| b.iter(|| solution(black_box(INPUT))));
}

fn d06(c: &mut Criterion) {
    use d06::{part2::solution, INPUT};
    c.bench_function("y2022::d06", |b| b.iter(|| solution(black_box(INPUT))));
}

fn d07(c: &mut Criterion) {
    use d07::{part2::solution, INPUT};
    c.bench_function("y2022::d07", |b| b.iter(|| solution(black_box(INPUT))));
}

fn d08(c: &mut Criterion) {
    use d08::{part2::solution, INPUT};
    c.bench_function("y2022::d08", |b| b.iter(|| solution(black_box(INPUT))));
}

fn d09(c: &mut Criterion) {
    use d09::{part2::solution, INPUT};
    c.bench_function("y2022::d09", |b| b.iter(|| solution(black_box(INPUT))));
}

fn d10(c: &mut Criterion) {
    use d10::{part2::solution, INPUT};
    c.bench_function("y2022::d10", |b| b.iter(|| solution(black_box(INPUT))));
}

fn d11(c: &mut Criterion) {
    use d11::{part2::solution, INPUT};
    c.bench_function("y2022::d11", |b| b.iter(|| solution(black_box(INPUT))));
}

fn d12(c: &mut Criterion) {
    use d12::{part1, part2, INPUT};
    c.bench_function("y2022::d12:part1", |b| {
        b.iter(|| part1::solution(black_box(INPUT)))
    });
    c.bench_function("y2022::d12:part2", |b| {
        b.iter(|| part2::solution(black_box(INPUT)))
    });
}

fn d13(c: &mut Criterion) {
    use d13::{part2::solution, INPUT};
    c.bench_function("y2022::d13", |b| b.iter(|| solution(black_box(INPUT))));
}

fn d14(c: &mut Criterion) {
    use d14::{part2::solution, INPUT};
    c.bench_function("y2022::d14", |b| b.iter(|| solution(black_box(INPUT))));
}

fn d15(c: &mut Criterion) {
    use d15::{part2::solution, INPUT};
    c.bench_function("y2022::d15", |b| b.iter(|| solution(black_box(INPUT), 4000000)));
}

fn d16(c: &mut Criterion) {
    use d16::{part1::solution, INPUT};
    c.bench_function("y2022::d16", |b| b.iter(|| solution(black_box(INPUT))));
}

fn d17(c: &mut Criterion) {
    use d17::{part1::solution, INPUT};
    c.bench_function("y2022::d17", |b| b.iter(|| solution(black_box(INPUT))));
}

fn d18(c: &mut Criterion) {
    use d18::{part1::solution, INPUT};
    c.bench_function("y2022::d18", |b| b.iter(|| solution(black_box(INPUT))));
}

fn d19(c: &mut Criterion) {
    use d19::{part1::solution, INPUT};
    c.bench_function("y2022::d19", |b| b.iter(|| solution(black_box(INPUT))));
}

fn d20(c: &mut Criterion) {
    use d20::{part2::solution, INPUT};
    c.bench_function("y2022::d20", |b| b.iter(|| solution(black_box(INPUT))));
}

fn d21(c: &mut Criterion) {
    use d21::{part2::solution, INPUT};
    c.bench_function("y2022::d21", |b| b.iter(|| solution(black_box(INPUT))));
}

fn d22(c: &mut Criterion) {
    use d22::{part2::solution, INPUT};
    c.bench_function("y2022::d22", |b| b.iter(|| solution::<50>(black_box(INPUT))));
}

fn d23(c: &mut Criterion) {
    use d23::{part2::solution, INPUT};
    c.bench_function("y2022::d23", |b| b.iter(|| solution(black_box(INPUT))));
}

fn d24(c: &mut Criterion) {
    use d24::{part2::solution, INPUT};
    c.bench_function("y2022::d24", |b| b.iter(|| solution(black_box(INPUT))));
}

fn d25(c: &mut Criterion) {
    use d25::{part1::solution, INPUT};
    c.bench_function("y2022::d25", |b| b.iter(|| solution(black_box(INPUT))));
}

criterion_group!(
    benches, d01, d02, d03, d04, d05, d06, d07, d08, d09, d10, d11, d12, //
    d13, d14, d15, d16, d17, d18, d19, d20, d21, d22, d23, d24, d25
);
