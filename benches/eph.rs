use std::hint::black_box;
use criterion::{criterion_group, criterion_main, Criterion};
use sofars::eph;

fn bench_eph(c: &mut Criterion) {
    let mut group = c.benchmark_group("eph");

    group.bench_function("epv00", |b| {
        b.iter(|| eph::epv00(black_box(2400000.5), black_box(53411.52501161)))
    });

    group.bench_function("moon98", |b| {
        b.iter(|| eph::moon98(black_box(2400000.5), black_box(43999.9)))
    });

    group.bench_function("plan94", |b| {
        b.iter(|| eph::plan94(black_box(2400000.5), black_box(43999.9), black_box(1)))
    });

    group.finish();
}

criterion_group!(benches, bench_eph);
criterion_main!(benches);
