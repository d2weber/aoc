use criterion::criterion_main;

mod y2022;
mod y2023;

criterion_main!(y2022::benches, y2023::benches);
