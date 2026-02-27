use std::hint::black_box;
use criterion::{criterion_group, criterion_main, Criterion};
use sofars::astro::*;

fn bench_astro(c: &mut Criterion) {
    let mut group = c.benchmark_group("astro");

    // Common test parameters
    let utc1 = 2456384.5;
    let utc2 = 0.969254051;
    let tt1 = 2456165.5;
    let tt2 = 0.401182685;
    let dut1 = 0.1550675;
    let elong = -0.527800806;
    let phi = -1.2345856;
    let hm = 2738.0;
    let xp = 2.47230737e-7;
    let yp = 1.82640464e-6;
    let phpa = 731.0;
    let tc = 12.8;
    let rh = 0.59;
    let wl = 0.55;

    let rc = 2.71;
    let dc = 0.174;
    let pr = 1e-5;
    let pd = 5e-6;
    let px = 0.1;
    let rv = 55.0;

    // pmsafe
    group.bench_function("pmsafe", |b| {
        b.iter(|| pmsafe(
            black_box(1.234), black_box(0.789), black_box(1e-5), black_box(-2e-5), black_box(1e-2), black_box(10.0),
            black_box(2400000.5), black_box(48348.5625), black_box(2400000.5), black_box(51544.5)
        ))
    });

    // atoi13
    group.bench_function("atoi13", |b| {
        b.iter(|| atoi13(
            black_box("R"), black_box(2.710085107986886201), black_box(0.1717653435758265198), 
            black_box(utc1), black_box(utc2), black_box(dut1), black_box(elong), black_box(phi), 
            black_box(hm), black_box(xp), black_box(yp), black_box(phpa), black_box(tc), black_box(rh), black_box(wl)
        ))
    });

    // atoc13
    group.bench_function("atoc13", |b| {
        b.iter(|| atoc13(
            black_box("R"), black_box(2.710085107986886201), black_box(0.1717653435758265198), 
            black_box(utc1), black_box(utc2), black_box(dut1), black_box(elong), black_box(phi), 
            black_box(hm), black_box(xp), black_box(yp), black_box(phpa), black_box(tc), black_box(rh), black_box(wl)
        ))
    });

    // ab
    let pnat = [-0.76, -0.60, -0.21];
    let v_ab = [2.1e-5, -8.9e-5, -3.8e-5];
    group.bench_function("ab", |b| {
        b.iter(|| ab(black_box(&pnat), black_box(&v_ab), black_box(0.999), black_box(0.999)))
    });

    // pvstar
    let pv_data = [[126668.5, 2136.7, -245251.2], [-0.004, -0.006, 0.011]];
    group.bench_function("pvstar", |b| {
        b.iter(|| pvstar(black_box(&pv_data)))
    });

    // pvtob
    group.bench_function("pvtob", |b| {
        let mut pv = [[0.0; 3]; 2];
        b.iter(|| pvtob(black_box(elong), black_box(phi), black_box(hm), black_box(xp), black_box(yp), black_box(1e-8), black_box(5.0), &mut pv))
    });

    // apcg
    let ebpv = [[0.9, -0.4, -0.1], [0.007, 0.014, 0.006]];
    let ehp = [0.9, -0.4, -0.1];
    group.bench_function("apcg", |b| {
        let mut astrom = IauAstrom::default();
        b.iter(|| apcg(black_box(tt1), black_box(tt2), black_box(&ebpv), black_box(&ehp), &mut astrom))
    });

    // apcg13
    group.bench_function("apcg13", |b| {
        let mut astrom = IauAstrom::default();
        b.iter(|| apcg13(black_box(tt1), black_box(tt2), &mut astrom))
    });

    // apci
    group.bench_function("apci", |b| {
        let mut astrom = IauAstrom::default();
        b.iter(|| apci(black_box(tt1), black_box(tt2), black_box(&ebpv), black_box(&ehp), black_box(0.001), black_box(-2e-5), black_box(3e-8), &mut astrom))
    });

    // apci13
    group.bench_function("apci13", |b| {
        let mut astrom = IauAstrom::default();
        let mut eo = 0.0;
        b.iter(|| apci13(black_box(tt1), black_box(tt2), &mut astrom, &mut eo))
    });

    // apcs
    let pv_apcs = [[-1836024.0, 1056607.0, -5998795.0], [-77.0, -133.0, 0.09]];
    group.bench_function("apcs", |b| {
        let mut astrom = IauAstrom::default();
        b.iter(|| apcs(black_box(tt1), black_box(tt2), black_box(&pv_apcs), black_box(&ebpv), black_box(&ehp), &mut astrom))
    });

    // apcs13
    group.bench_function("apcs13", |b| {
        let mut astrom = IauAstrom::default();
        b.iter(|| apcs13(black_box(tt1), black_box(tt2), black_box(&pv_apcs), &mut astrom))
    });

    // aper
    group.bench_function("aper", |b| {
        let mut astrom = IauAstrom::default();
        b.iter(|| aper(black_box(5.678), &mut astrom))
    });

    // aper13
    group.bench_function("aper13", |b| {
        let mut astrom = IauAstrom::default();
        b.iter(|| aper13(black_box(tt1), black_box(tt2), &mut astrom))
    });

    // atcc13
    group.bench_function("atcc13", |b| {
        b.iter(|| atcc13(black_box(rc), black_box(dc), black_box(pr), black_box(pd), black_box(px), black_box(rv), black_box(tt1), black_box(tt2)))
    });

    // apco
    group.bench_function("apco", |b| {
        let mut astrom = IauAstrom::default();
        b.iter(|| apco(black_box(tt1), black_box(tt2), black_box(&ebpv), black_box(&ehp), black_box(0.001), black_box(-2e-5), black_box(3e-8), black_box(3.14), black_box(elong), black_box(phi), black_box(hm), black_box(xp), black_box(yp), black_box(0.0), black_box(0.0002), black_box(-2e-7), &mut astrom))
    });

    // apco13
    group.bench_function("apco13", |b| {
        let mut astrom = IauAstrom::default();
        let mut eo = 0.0;
        b.iter(|| apco13(black_box(utc1), black_box(utc2), black_box(dut1), black_box(elong), black_box(phi), black_box(hm), black_box(xp), black_box(yp), black_box(phpa), black_box(tc), black_box(rh), black_box(wl), &mut astrom, &mut eo))
    });

    // atci13
    group.bench_function("atci13", |b| {
        b.iter(|| atci13(black_box(rc), black_box(dc), black_box(pr), black_box(pd), black_box(px), black_box(rv), black_box(tt1), black_box(tt2)))
    });

    // atciq
    group.bench_function("atciq", |b| {
        let mut astrom = IauAstrom::default();
        let mut eo = 0.0;
        apci13(tt1, tt2, &mut astrom, &mut eo);
        b.iter(|| atciq(black_box(rc), black_box(dc), black_box(pr), black_box(pd), black_box(px), black_box(rv), &mut astrom))
    });

    // atciqn
    let bodies = [
        IauLdBody::new(0.00028574, 3e-10, [[-7.81, -5.61, -1.98], [0.003, -0.004, -0.001]]),
        IauLdBody::new(0.00095435, 3e-9, [[0.738, 4.636, 1.969], [-0.007, 0.001, 0.0007]]),
        IauLdBody::new(1.0, 6e-6, [[-0.0007, -0.0023, -0.001], [0.000006, -0.0000003, -0.0000002]])
    ];
    group.bench_function("atciqn", |b| {
        let mut astrom = IauAstrom::default();
        let mut eo = 0.0;
        apci13(tt1, tt2, &mut astrom, &mut eo);
        b.iter(|| atciqn(black_box(rc), black_box(dc), black_box(pr), black_box(pd), black_box(px), black_box(rv), &mut astrom, black_box(3), black_box(&bodies)))
    });

    // atciqz
    group.bench_function("atciqz", |b| {
        let mut astrom = IauAstrom::default();
        let mut eo = 0.0;
        apci13(tt1, tt2, &mut astrom, &mut eo);
        b.iter(|| atciqz(black_box(rc), black_box(dc), &mut astrom))
    });

    // atco13
    group.bench_function("atco13", |b| {
        b.iter(|| atco13(black_box(rc), black_box(dc), black_box(pr), black_box(pd), black_box(px), black_box(rv), black_box(utc1), black_box(utc2), black_box(dut1), black_box(elong), black_box(phi), black_box(hm), black_box(xp), black_box(yp), black_box(phpa), black_box(tc), black_box(rh), black_box(wl)))
    });

    // atic13
    group.bench_function("atic13", |b| {
        b.iter(|| atic13(black_box(rc), black_box(dc), black_box(tt1), black_box(tt2)))
    });

    // aticq
    group.bench_function("aticq", |b| {
        let mut astrom = IauAstrom::default();
        let mut eo = 0.0;
        apci13(tt1, tt2, &mut astrom, &mut eo);
        b.iter(|| aticq(black_box(rc), black_box(dc), &mut astrom))
    });

    // aticqn
    group.bench_function("aticqn", |b| {
        let mut astrom = IauAstrom::default();
        let mut eo = 0.0;
        apci13(tt1, tt2, &mut astrom, &mut eo);
        b.iter(|| aticqn(black_box(rc), black_box(dc), &mut astrom, black_box(3), black_box(&bodies)))
    });

    // atio13
    group.bench_function("atio13", |b| {
        b.iter(|| atio13(black_box(rc), black_box(dc), black_box(utc1), black_box(utc2), black_box(dut1), black_box(elong), black_box(phi), black_box(hm), black_box(xp), black_box(yp), black_box(phpa), black_box(tc), black_box(rh), black_box(wl)))
    });

    // ld
    let p_ld = [-0.76, -0.60, -0.21];
    let q_ld = [-0.76, -0.60, -0.21];
    let e_ld = [0.76, 0.60, 0.21];
    group.bench_function("ld", |b| {
        b.iter(|| ld(black_box(0.00028), black_box(p_ld), black_box(q_ld), black_box(e_ld), black_box(8.9), black_box(3e-10)))
    });

    // ldn
    group.bench_function("ldn", |b| {
        b.iter(|| ldn(black_box(3), black_box(&bodies), black_box(&ehp), black_box(&p_ld)))
    });

    // ldsun
    group.bench_function("ldsun", |b| {
        b.iter(|| ldsun(black_box(p_ld), black_box(ehp), black_box(0.999)))
    });

    // starpv
    group.bench_function("starpv", |b| {
        b.iter(|| starpv(black_box(rc), black_box(dc), black_box(pr), black_box(pd), black_box(px), black_box(rv)))
    });

    group.finish();
}

criterion_group!(benches, bench_astro);
criterion_main!(benches);
