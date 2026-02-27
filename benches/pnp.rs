use std::hint::black_box;
use criterion::{criterion_group, criterion_main, Criterion};
use sofars::pnp::*;

fn bench_pnp(c: &mut Criterion) {
    let mut group = c.benchmark_group("pnp");

    // Common test parameters
    let dj1 = 2400000.5;
    let dj2 = 53736.0;
    let x = 0.5791308486706011000e-3;
    let y = 0.4020579816732961219e-4;
    let s = -0.1220040848472271978e-7;
    let r_bpn = [[1.0, 0.0, 0.0], [0.0, 1.0, 0.0], [0.0, 0.0, 1.0]];

    group.bench_function("bi00", |b| {
        b.iter(|| bi00())
    });

    group.bench_function("bp00", |b| {
        b.iter(|| bp00(black_box(dj1), black_box(50123.9999)))
    });

    group.bench_function("bp06", |b| {
        b.iter(|| bp06(black_box(dj1), black_box(50123.9999)))
    });

    group.bench_function("bpn2xy", |b| {
        b.iter(|| bpn2xy(black_box(&r_bpn)))
    });

    group.bench_function("c2i00a", |b| {
        b.iter(|| c2i00a(black_box(dj1), black_box(dj2)))
    });

    group.bench_function("c2i00b", |b| {
        b.iter(|| c2i00b(black_box(dj1), black_box(dj2)))
    });

    group.bench_function("c2i06a", |b| {
        b.iter(|| c2i06a(black_box(dj1), black_box(dj2)))
    });

    group.bench_function("c2ibpn", |b| {
        b.iter(|| c2ibpn(black_box(dj1), black_box(dj2), black_box(&r_bpn)))
    });

    group.bench_function("c2ixy", |b| {
        b.iter(|| c2ixy(black_box(dj1), black_box(dj2), black_box(x), black_box(y)))
    });

    group.bench_function("c2ixys", |b| {
        b.iter(|| c2ixys(black_box(x), black_box(y), black_box(s)))
    });

    group.bench_function("c2t00a", |b| {
        b.iter(|| c2t00a(black_box(dj1), black_box(dj2), black_box(dj1), black_box(dj2), black_box(0.0), black_box(0.0)))
    });

    group.bench_function("c2t00b", |b| {
        b.iter(|| c2t00b(black_box(dj1), black_box(dj2), black_box(dj1), black_box(dj2), black_box(0.0), black_box(0.0)))
    });

    group.bench_function("c2t06a", |b| {
        b.iter(|| c2t06a(black_box(dj1), black_box(dj2), black_box(dj1), black_box(dj2), black_box(0.0), black_box(0.0)))
    });

    group.bench_function("c2tcio", |b| {
        b.iter(|| c2tcio(black_box(&r_bpn), black_box(3.14), black_box(&r_bpn)))
    });

    group.bench_function("c2teqx", |b| {
        b.iter(|| c2teqx(black_box(&r_bpn), black_box(3.14), black_box(&r_bpn)))
    });

    group.bench_function("c2tpe", |b| {
        b.iter(|| c2tpe(black_box(dj1), black_box(dj2), black_box(dj1), black_box(dj2), black_box(0.001), black_box(-0.002), black_box(0.4), black_box(-0.1)))
    });

    group.bench_function("c2txy", |b| {
        b.iter(|| c2txy(black_box(dj1), black_box(dj2), black_box(dj1), black_box(dj2), black_box(x), black_box(y), black_box(0.0), black_box(0.0)))
    });

    group.bench_function("eo06a", |b| {
        b.iter(|| eo06a(black_box(dj1), black_box(dj2)))
    });

    group.bench_function("eors", |b| {
        b.iter(|| eors(black_box(&r_bpn), black_box(s)))
    });

    group.bench_function("fw2m", |b| {
        b.iter(|| fw2m(black_box(0.1), black_box(0.2), black_box(0.3), black_box(0.4)))
    });

    group.bench_function("fw2xy", |b| {
        b.iter(|| fw2xy(black_box(0.1), black_box(0.2), black_box(0.3), black_box(0.4)))
    });

    group.bench_function("ltp", |b| {
        b.iter(|| ltp(black_box(2000.0)))
    });

    group.bench_function("ltpb", |b| {
        b.iter(|| ltpb(black_box(2000.0)))
    });

    group.bench_function("ltpecl", |b| {
        b.iter(|| ltpecl(black_box(2000.0)))
    });

    group.bench_function("ltpequ", |b| {
        b.iter(|| ltpequ(black_box(2000.0)))
    });

    group.bench_function("num00a", |b| {
        b.iter(|| num00a(black_box(dj1), black_box(dj2)))
    });

    group.bench_function("num00b", |b| {
        b.iter(|| num00b(black_box(dj1), black_box(dj2)))
    });

    group.bench_function("num06a", |b| {
        b.iter(|| num06a(black_box(dj1), black_box(dj2)))
    });

    group.bench_function("numat", |b| {
        b.iter(|| numat(black_box(0.4), black_box(0.001), black_box(-0.002)))
    });

    group.bench_function("nut00a", |b| {
        b.iter(|| nut00a(black_box(dj1), black_box(dj2)))
    });

    group.bench_function("nut00b", |b| {
        b.iter(|| nut00b(black_box(dj1), black_box(dj2)))
    });

    group.bench_function("nut06a", |b| {
        b.iter(|| nut06a(black_box(dj1), black_box(dj2)))
    });

    group.bench_function("nut80", |b| {
        b.iter(|| nut80(black_box(dj1), black_box(dj2)))
    });

    group.bench_function("nutm80", |b| {
        b.iter(|| nutm80(black_box(dj1), black_box(dj2)))
    });

    group.bench_function("obl06", |b| {
        b.iter(|| obl06(black_box(dj1), black_box(dj2)))
    });

    group.bench_function("obl80", |b| {
        b.iter(|| obl80(black_box(dj1), black_box(dj2)))
    });

    group.bench_function("p06e", |b| {
        b.iter(|| p06e(black_box(dj1), black_box(dj2)))
    });

    group.bench_function("pb06", |b| {
        b.iter(|| pb06(black_box(dj1), black_box(dj2)))
    });

    group.bench_function("pfw06", |b| {
        b.iter(|| pfw06(black_box(dj1), black_box(dj2)))
    });

    group.bench_function("pmat00", |b| {
        b.iter(|| pmat00(black_box(dj1), black_box(dj2)))
    });

    group.bench_function("pmat06", |b| {
        b.iter(|| pmat06(black_box(dj1), black_box(dj2)))
    });

    group.bench_function("pmat76", |b| {
        b.iter(|| pmat76(black_box(dj1), black_box(dj2)))
    });

    group.bench_function("pn00", |b| {
        b.iter(|| pn00(black_box(dj1), black_box(dj2), black_box(0.001), black_box(-0.002)))
    });

    group.bench_function("pn00a", |b| {
        b.iter(|| pn00a(black_box(dj1), black_box(dj2)))
    });

    group.bench_function("pn00b", |b| {
        b.iter(|| pn00b(black_box(dj1), black_box(dj2)))
    });

    group.bench_function("pn06", |b| {
        b.iter(|| pn06(black_box(dj1), black_box(dj2), black_box(0.001), black_box(-0.002)))
    });

    group.bench_function("pn06a", |b| {
        b.iter(|| pn06a(black_box(dj1), black_box(dj2)))
    });

    group.bench_function("pnm00a", |b| {
        b.iter(|| pnm00a(black_box(dj1), black_box(dj2)))
    });

    group.bench_function("pnm00b", |b| {
        b.iter(|| pnm00b(black_box(dj1), black_box(dj2)))
    });

    group.bench_function("pnm06a", |b| {
        b.iter(|| pnm06a(black_box(dj1), black_box(dj2)))
    });

    group.bench_function("pnm80", |b| {
        b.iter(|| pnm80(black_box(dj1), black_box(dj2)))
    });

    group.bench_function("pom00", |b| {
        b.iter(|| pom00(black_box(0.001), black_box(-0.002), black_box(3e-8)))
    });

    group.bench_function("pr00", |b| {
        b.iter(|| pr00(black_box(dj1), black_box(dj2)))
    });

    group.bench_function("prec76", |b| {
        b.iter(|| prec76(black_box(dj1), black_box(dj2), black_box(dj1), black_box(dj2)))
    });

    group.bench_function("s00", |b| {
        b.iter(|| s00(black_box(dj1), black_box(dj2), black_box(x), black_box(y)))
    });

    group.bench_function("s00a", |b| {
        b.iter(|| s00a(black_box(dj1), black_box(dj2)))
    });

    group.bench_function("s00b", |b| {
        b.iter(|| s00b(black_box(dj1), black_box(dj2)))
    });

    group.bench_function("s06", |b| {
        b.iter(|| s06(black_box(dj1), black_box(dj2), black_box(x), black_box(y)))
    });

    group.bench_function("s06a", |b| {
        b.iter(|| s06a(black_box(dj1), black_box(dj2)))
    });

    group.bench_function("sp00", |b| {
        b.iter(|| sp00(black_box(dj1), black_box(dj2)))
    });

    group.bench_function("xy06", |b| {
        b.iter(|| xy06(black_box(dj1), black_box(dj2)))
    });

    group.bench_function("xys00a", |b| {
        b.iter(|| xys00a(black_box(dj1), black_box(dj2)))
    });

    group.bench_function("xys00b", |b| {
        b.iter(|| xys00b(black_box(dj1), black_box(dj2)))
    });

    group.bench_function("xys06a", |b| {
        b.iter(|| xys06a(black_box(dj1), black_box(dj2)))
    });

    group.finish();
}

criterion_group!(benches, bench_pnp);
criterion_main!(benches);
