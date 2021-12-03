use criterion::{black_box, criterion_group, criterion_main, Criterion};
use day2::*;

fn bench_part_1(c: &mut Criterion) {
    let input = include_str!("../src/part_1.txt");
    c.bench_function("part_1", |b| b.iter(|| part_1(black_box(input))));
}

fn bench_part_2(c: &mut Criterion) {
    let input = include_str!("../src/part_1.txt");
    c.bench_function("part_2", |b| b.iter(|| part_2(black_box(input))));
}

criterion_group!(benches, bench_part_1, bench_part_2,);
criterion_main!(benches);
