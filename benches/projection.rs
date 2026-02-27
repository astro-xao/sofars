use std::hint::black_box;
use criterion::{criterion_group, criterion_main, Criterion};
use sofars::projection::*;

fn bench_projection(c: &mut Criterion) {
    let mut group = c.benchmark_group("projection");

    let xi = -0.03;
    let eta = 0.07;
    let ra = 1.3;
    let dec = 1.5;
    let v = [0.07008, 0.9296, 0.3616]; // s2c(1.3, 1.5)

    group.bench_function("tpors", |b| {
        b.iter(|| tpors(black_box(xi), black_box(eta), black_box(ra), black_box(dec)))
    });

    group.bench_function("tporv", |b| {
        b.iter(|| tporv(black_box(xi), black_box(eta), black_box(v)))
    });

    group.bench_function("tpsts", |b| {
        b.iter(|| tpsts(black_box(xi), black_box(eta), black_box(2.3), black_box(1.5)))
    });

    group.bench_function("tpstv", |b| {
        b.iter(|| tpstv(black_box(xi), black_box(eta), black_box(v)))
    });

    group.bench_function("tpxes", |b| {
        b.iter(|| tpxes(black_box(ra), black_box(1.55), black_box(2.3), black_box(1.5)))
    });

    group.bench_function("tpxev", |b| {
        b.iter(|| tpxev(black_box(v), black_box(v)))
    });

    group.finish();
}

criterion_group!(benches, bench_projection);
criterion_main!(benches);
