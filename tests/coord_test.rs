use sofars::coords::ae2hd;

#[test]
fn test_ae2hd() {
    let a = 5.5;
    let e = 1.1;
    let p = 0.7;

    let [h, d] = ae2hd(a, e, p);

    assert!((h - 0.5933291115507309663).abs() < 1e-14, "iauAe2hd h");
    assert!((d - 0.9613934761647817620).abs() < 1e-14, "iauAe2hd d");
}