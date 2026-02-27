use std::hint::black_box;
use criterion::{criterion_group, criterion_main, Criterion};
use sofars::erst;

fn bench_erst(c: &mut Criterion) {
    let mut group = c.benchmark_group("erst");

    // Common test parameters
    let dj1 = 2400000.5;
    let dj2 = 53736.0;
    let epsa = 0.4090789763356509900;
    let dpsi = -0.9630909107115582393e-5;

    group.bench_function("ee00", |b| {
        b.iter(|| erst::ee00(black_box(dj1), black_box(dj2), black_box(epsa), black_box(dpsi)))
    });

    group.bench_function("ee00a", |b| {
        b.iter(|| erst::ee00a(black_box(dj1), black_box(dj2)))
    });

    group.bench_function("ee00b", |b| {
        b.iter(|| erst::ee00b(black_box(dj1), black_box(dj2)))
    });

    group.bench_function("ee06a", |b| {
        b.iter(|| erst::ee06a(black_box(dj1), black_box(dj2)))
    });

    group.bench_function("eect00", |b| {
        b.iter(|| erst::eect00(black_box(dj1), black_box(dj2)))
    });

    group.bench_function("eqeq94", |b| {
        b.iter(|| erst::eqeq94(black_box(dj1), black_box(41234.0)))
    });

    group.bench_function("era00", |b| {
        b.iter(|| erst::era00(black_box(dj1), black_box(54388.0)))
    });

    group.bench_function("gmst00", |b| {
        b.iter(|| erst::gmst00(black_box(dj1), black_box(dj2), black_box(dj1), black_box(dj2)))
    });

    group.bench_function("gmst06", |b| {
        b.iter(|| erst::gmst06(black_box(dj1), black_box(dj2), black_box(dj1), black_box(dj2)))
    });

    group.bench_function("gmst82", |b| {
        b.iter(|| erst::gmst82(black_box(dj1), black_box(dj2)))
    });

    group.bench_function("gst00a", |b| {
        b.iter(|| erst::gst00a(black_box(dj1), black_box(dj2), black_box(dj1), black_box(dj2)))
    });

    group.bench_function("gst00b", |b| {
        b.iter(|| erst::gst00b(black_box(dj1), black_box(dj2)))
    });

    let rnpb = [
        [0.9999989440476103608, -0.1332881761240011518e-2, -0.5790767434730085097e-3],
        [0.1332858254308954453e-2, 0.9999991109044505944, -0.4097782710401555759e-4],
        [0.5791308472168153320e-3, 0.4020595661593994396e-4, 0.9999998314954572365],
    ];

    group.bench_function("gst06", |b| {
        b.iter(|| erst::gst06(black_box(dj1), black_box(dj2), black_box(dj1), black_box(dj2), black_box(&rnpb)))
    });

    group.bench_function("gst06a", |b| {
        b.iter(|| erst::gst06a(black_box(dj1), black_box(dj2), black_box(dj1), black_box(dj2)))
    });

    group.bench_function("gst94", |b| {
        b.iter(|| erst::gst94(black_box(dj1), black_box(dj2)))
    });

    group.finish();
}

criterion_group!(benches, bench_erst);
criterion_main!(benches);
