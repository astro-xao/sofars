use std::hint::black_box;
use criterion::{criterion_group, criterion_main, Criterion};
use sofars::star::*;

fn bench_star(c: &mut Criterion) {
    let mut group = c.benchmark_group("star");

    let r = 0.07626;
    let d = -1.1374;
    let dr = 0.197e-4;
    let dd = 0.565e-5;
    let p = 0.134;
    let v = 8.7;
    let bepoch = 1954.677;
    let date1 = 2400000.5;
    let date2 = 54479.0;

    group.bench_function("fk425", |b| {
        b.iter(|| fk425(black_box(r), black_box(d), black_box(dr), black_box(dd), black_box(p), black_box(v)))
    });

    group.bench_function("fk45z", |b| {
        b.iter(|| fk45z(black_box(r), black_box(d), black_box(bepoch)))
    });

    group.bench_function("fk524", |b| {
        b.iter(|| fk524(black_box(r), black_box(d), black_box(dr), black_box(dd), black_box(p), black_box(v)))
    });

    group.bench_function("fk52h", |b| {
        b.iter(|| fk52h(black_box(r), black_box(d), black_box(dr), black_box(dd), black_box(p), black_box(v)))
    });

    group.bench_function("fk54z", |b| {
        b.iter(|| fk54z(black_box(r), black_box(d), black_box(bepoch)))
    });

    group.bench_function("fk5hip", |b| {
        b.iter(|| fk5hip())
    });

    group.bench_function("fk5hz", |b| {
        b.iter(|| fk5hz(black_box(r), black_box(d), black_box(date1), black_box(date2)))
    });

    group.bench_function("h2fk5", |b| {
        b.iter(|| h2fk5(black_box(r), black_box(d), black_box(dr), black_box(dd), black_box(p), black_box(v)))
    });

    group.bench_function("hfk5z", |b| {
        b.iter(|| hfk5z(black_box(r), black_box(d), black_box(date1), black_box(date2)))
    });

    group.finish();
}

criterion_group!(benches, bench_star);
criterion_main!(benches);
