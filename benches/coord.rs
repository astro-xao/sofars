use std::hint::black_box;
use criterion::{criterion_group, criterion_main, Criterion};
use sofars::coords::*;

fn bench_coord(c: &mut Criterion) {
    let mut group = c.benchmark_group("coord");

    let dj1 = 2456165.5;
    let dj2 = 0.401182685;
    let xyz = [2e6, 3e6, 5.244e6];

    group.bench_function("ae2hd", |b| {
        b.iter(|| ae2hd(black_box(5.5), black_box(1.1), black_box(0.7)))
    });

    group.bench_function("hd2ae", |b| {
        b.iter(|| hd2ae(black_box(1.1), black_box(1.2), black_box(0.3)))
    });

    group.bench_function("hd2pa", |b| {
        b.iter(|| hd2pa(black_box(1.1), black_box(1.2), black_box(0.3)))
    });

    group.bench_function("eceq06", |b| {
        b.iter(|| eceq06(black_box(dj1), black_box(dj2), black_box(5.1), black_box(-0.9)))
    });

    group.bench_function("ecm06", |b| {
        b.iter(|| ecm06(black_box(dj1), black_box(dj2)))
    });

    group.bench_function("eqec06", |b| {
        b.iter(|| eqec06(black_box(1234.5), black_box(2440000.5), black_box(1.234), black_box(0.987)))
    });

    group.bench_function("lteceq", |b| {
        b.iter(|| lteceq(black_box(2500.0), black_box(1.5), black_box(0.6)))
    });

    group.bench_function("ltecm", |b| {
        b.iter(|| ltecm(black_box(-3000.0)))
    });

    group.bench_function("lteqec", |b| {
        b.iter(|| lteqec(black_box(-1500.0), black_box(1.234), black_box(0.987)))
    });

    group.bench_function("g2icrs", |b| {
        b.iter(|| g2icrs(black_box(5.585), black_box(-0.785)))
    });

    group.bench_function("icrs2g", |b| {
        b.iter(|| icrs2g(black_box(5.933), black_box(-1.178)))
    });

    group.bench_function("eform", |b| {
        b.iter(|| eform(black_box(1)))
    });

    group.bench_function("gc2gd", |b| {
        b.iter(|| gc2gd(black_box(1), black_box(xyz)))
    });

    group.bench_function("gc2gde", |b| {
        b.iter(|| gc2gde(black_box(6378136.0), black_box(0.0033528), black_box(xyz)))
    });

    group.bench_function("gd2gc", |b| {
        b.iter(|| gd2gc(black_box(1), black_box(3.1), black_box(-0.5), black_box(2500.0)))
    });

    group.bench_function("gd2gce", |b| {
        b.iter(|| gd2gce(black_box(6378136.0), black_box(0.0033528), black_box(3.1), black_box(-0.5), black_box(2500.0)))
    });

    group.finish();
}

criterion_group!(benches, bench_coord);
criterion_main!(benches);
