use criterion::{black_box, criterion_group, criterion_main, Criterion};
use day1::*;

fn bench_part_1(c: &mut Criterion) {
    let input = include_str!("../src/part_1.txt");
    c.bench_function("part_1", |b| b.iter(|| part_1(black_box(input))));
}

fn bench_part_2(c: &mut Criterion) {
    let input = include_str!("../src/part_1.txt");
    c.bench_function("part_2", |b| b.iter(|| part_2(black_box(input))));
}

fn bench_part_1_no_mutate(c: &mut Criterion) {
    let input = include_str!("../src/part_1.txt");
    c.bench_function("part_1_no_mutate", |b| {
        b.iter(|| part_1_no_mutate(black_box(input)))
    });
}

fn bench_part_1_no_option(c: &mut Criterion) {
    let input = include_str!("../src/part_1.txt");
    c.bench_function("part_1_no_option", |b| {
        b.iter(|| part_1_no_option(black_box(input)))
    });
}

fn bench_part_1_opt_pat(c: &mut Criterion) {
    let input = include_str!("../src/part_1.txt");
    c.bench_function("part_1_opt_pat", |b| {
        b.iter(|| part_1_opt_pat(black_box(input)))
    });
}

criterion_group!(
    benches,
    bench_part_1,
    bench_part_2,
    bench_part_1_no_mutate,
    bench_part_1_no_option,
    bench_part_1_opt_pat
);
criterion_main!(benches);
