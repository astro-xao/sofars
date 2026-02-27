use std::hint::black_box;
use criterion::{criterion_group, criterion_main, Criterion};
use sofars::cal::*;

fn bench_calendars(c: &mut Criterion) {
    let mut group = c.benchmark_group("calendars");

    group.bench_function("cal2jd", |b| {
        b.iter(|| cal2jd(black_box(2003), black_box(6), black_box(1)))
    });

    group.bench_function("epb", |b| {
        b.iter(|| epb(black_box(2415019.8135), black_box(30103.18648)))
    });

    group.bench_function("epb2jd", |b| {
        b.iter(|| epb2jd(black_box(1957.3)))
    });

    group.bench_function("epj", |b| {
        b.iter(|| epj(black_box(2451545.0), black_box(-7392.5)))
    });

    group.bench_function("epj2jd", |b| {
        b.iter(|| epj2jd(black_box(1996.8)))
    });

    group.bench_function("jd2cal", |b| {
        b.iter(|| jd2cal(black_box(2400000.5), black_box(50123.9999)))
    });

    group.bench_function("jdcalf", |b| {
        b.iter(|| jdcalf(black_box(4), black_box(2400000.5), black_box(50123.9999)))
    });

    group.finish();
}

criterion_group!(benches, bench_calendars);
criterion_main!(benches);
