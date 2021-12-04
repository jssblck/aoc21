use aoc21::*;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn day1_part_1(c: &mut Criterion) {
    let input = include_str!("../src/inputs/day1.txt");
    c.bench_function("part_1", |b| b.iter(|| day1::part_1(black_box(input))));
}

fn day1_part_2(c: &mut Criterion) {
    let input = include_str!("../src/inputs/day1.txt");
    c.bench_function("part_2", |b| b.iter(|| day1::part_2(black_box(input))));
}

fn day1_part_1_no_mutate(c: &mut Criterion) {
    let input = include_str!("../src/inputs/day1.txt");
    c.bench_function("part_1_no_mutate", |b| {
        b.iter(|| day1::part_1_no_mutate(black_box(input)))
    });
}

fn day1_part_1_no_option(c: &mut Criterion) {
    let input = include_str!("../src/inputs/day1.txt");
    c.bench_function("part_1_no_option", |b| {
        b.iter(|| day1::part_1_no_option(black_box(input)))
    });
}

fn day1_part_1_opt_pat(c: &mut Criterion) {
    let input = include_str!("../src/inputs/day1.txt");
    c.bench_function("part_1_opt_pat", |b| {
        b.iter(|| day1::part_1_opt_pat(black_box(input)))
    });
}

fn day2_part_1(c: &mut Criterion) {
    let input = include_str!("../src/inputs/day2.txt");
    c.bench_function("part_1", |b| b.iter(|| day2::part_1(black_box(input))));
}

fn day2_part_2(c: &mut Criterion) {
    let input = include_str!("../src/inputs/day2.txt");
    c.bench_function("part_2", |b| b.iter(|| day2::part_2(black_box(input))));
}

criterion_group!(
    benches,
    day1_part_1,
    day1_part_2,
    day1_part_1_no_mutate,
    day1_part_1_no_option,
    day1_part_1_opt_pat,
    day2_part_1,
    day2_part_2
);
criterion_main!(benches);
