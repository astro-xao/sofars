use sofars::coords::{ae2hd, hd2ae, hd2pa};

#[test]
fn test_ae2hd() {
    let a = 5.5;
    let e = 1.1;
    let p = 0.7;

    let [h, d] = ae2hd(a, e, p);

    assert!((h - 0.5933291115507309663).abs() < 1e-14, "iauAe2hd h");
    assert!((d - 0.9613934761647817620).abs() < 1e-14, "iauAe2hd d");
}

#[test]
fn test_hd2ae() {
    let h = 1.1;
    let d = 1.2;
    let p = 0.3;

    let [a, e] = hd2ae(h, d, p);

    assert!((a - 5.916889243730066194).abs() < 1e-13, "iauHd2ae a");
    assert!((e - 0.4472186304990486228).abs() < 1e-14, "iauHd2ae e");
}

#[test]
fn test_hd2pa() {
    let h = 1.1;
    let d = 1.2;
    let p = 0.3;

    let q = hd2pa(h, d, p);

    assert!((q - 1.906227428001995580).abs() < 1e-13, "iauHd2pa q");
}
