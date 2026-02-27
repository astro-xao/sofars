use std::hint::black_box;
use criterion::{criterion_group, criterion_main, Criterion};
use sofars::ts;

fn bench_ts(c: &mut Criterion) {
    let mut group = c.benchmark_group("ts");

    // Common test parameters
    let dj1 = 2453750.5;
    let dj2 = 0.892482639;
    let dut1 = 0.3341;
    let dtti = 64.8499;
    let dtr = -0.000201;

    group.bench_function("d2dtf", |b| {
        b.iter(|| ts::d2dtf(black_box("UTC"), black_box(5), black_box(2400000.5), black_box(49533.99999)))
    });

    group.bench_function("dat", |b| {
        b.iter(|| ts::dat(black_box(2017), black_box(9), black_box(1), black_box(0.0)))
    });

    group.bench_function("ut1tt", |b| {
        b.iter(|| ts::ut1tt(black_box(dj1), black_box(0.892104561), black_box(dtti)))
    });

    group.bench_function("dtdb", |b| {
        b.iter(|| ts::dtdb(black_box(2448939.5), black_box(0.123), black_box(0.76543), black_box(5.0123), black_box(5525.242), black_box(3190.0)))
    });

    group.bench_function("dtf2d", |b| {
        b.iter(|| ts::dtf2d(black_box("UTC"), black_box(1994), black_box(6), black_box(30), black_box(23), black_box(59), black_box(60.13599)))
    });

    group.bench_function("taitt", |b| {
        b.iter(|| ts::taitt(black_box(dj1), black_box(dj2)))
    });

    group.bench_function("taiut1", |b| {
        b.iter(|| ts::taiut1(black_box(dj1), black_box(dj2), black_box(-32.6659)))
    });

    group.bench_function("utctai", |b| {
        b.iter(|| ts::utctai(black_box(dj1), black_box(0.892100694)))
    });

    group.bench_function("utcut1", |b| {
        b.iter(|| ts::utcut1(black_box(dj1), black_box(0.892100694), black_box(dut1)))
    });

    group.bench_function("taiutc", |b| {
        b.iter(|| ts::taiutc(black_box(dj1), black_box(dj2)))
    });

    group.bench_function("tcbtdb", |b| {
        b.iter(|| ts::tcbtdb(black_box(dj1), black_box(0.893019599)))
    });

    group.bench_function("tcgtt", |b| {
        b.iter(|| ts::tcgtt(black_box(dj1), black_box(0.892862531)))
    });

    group.bench_function("tdbtcb", |b| {
        b.iter(|| ts::tdbtcb(black_box(dj1), black_box(0.892855137)))
    });

    group.bench_function("tdbtt", |b| {
        b.iter(|| ts::tdbtt(black_box(dj1), black_box(0.892855137), black_box(dtr)))
    });

    group.bench_function("tttai", |b| {
        b.iter(|| ts::tttai(black_box(dj1), black_box(dj2)))
    });

    group.bench_function("tttcg", |b| {
        b.iter(|| ts::tttcg(black_box(dj1), black_box(dj2)))
    });

    group.bench_function("tttdb", |b| {
        b.iter(|| ts::tttdb(black_box(dj1), black_box(0.892855139), black_box(dtr)))
    });

    group.bench_function("ttut1", |b| {
        b.iter(|| ts::ttut1(black_box(dj1), black_box(0.892855139), black_box(dtti)))
    });

    group.bench_function("ut1tai", |b| {
        b.iter(|| ts::ut1tai(black_box(dj1), black_box(0.892104561), black_box(-32.6659)))
    });

    group.bench_function("ut1utc", |b| {
        b.iter(|| ts::ut1utc(black_box(dj1), black_box(0.892104561), black_box(dut1)))
    });

    group.finish();
}

criterion_group!(benches, bench_ts);
criterion_main!(benches);
