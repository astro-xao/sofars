use std::hint::black_box;
use criterion::{criterion_group, criterion_main, Criterion};
use sofars::fundargs;

fn bench_fundargs(c: &mut Criterion) {
    let mut group = c.benchmark_group("fundargs");
    let t = 0.80;

    group.bench_function("fad03", |b| {
        b.iter(|| fundargs::fad03(black_box(t)))
    });

    group.bench_function("fae03", |b| {
        b.iter(|| fundargs::fae03(black_box(t)))
    });

    group.bench_function("faf03", |b| {
        b.iter(|| fundargs::faf03(black_box(t)))
    });

    group.bench_function("faju03", |b| {
        b.iter(|| fundargs::faju03(black_box(t)))
    });

    group.bench_function("fal03", |b| {
        b.iter(|| fundargs::fal03(black_box(t)))
    });

    group.bench_function("falp03", |b| {
        b.iter(|| fundargs::falp03(black_box(t)))
    });

    group.bench_function("fama03", |b| {
        b.iter(|| fundargs::fama03(black_box(t)))
    });

    group.bench_function("fame03", |b| {
        b.iter(|| fundargs::fame03(black_box(t)))
    });

    group.bench_function("fane03", |b| {
        b.iter(|| fundargs::fane03(black_box(t)))
    });

    group.bench_function("faom03", |b| {
        b.iter(|| fundargs::faom03(black_box(t)))
    });

    group.bench_function("fapa03", |b| {
        b.iter(|| fundargs::fapa03(black_box(t)))
    });

    group.bench_function("fasa03", |b| {
        b.iter(|| fundargs::fasa03(black_box(t)))
    });

    group.bench_function("faur03", |b| {
        b.iter(|| fundargs::faur03(black_box(t)))
    });

    group.bench_function("fave03", |b| {
        b.iter(|| fundargs::fave03(black_box(t)))
    });

    group.finish();
}

criterion_group!(benches, bench_fundargs);
criterion_main!(benches);
