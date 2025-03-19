use sofars::ts;
#[test]
fn test_dat() {
    let mut deltat: f64;

    deltat = ts::dat(2003, 6, 1, 0.0).unwrap();
    assert!((deltat - 32.0).abs() < 1e-12, "iauDat d1");

    deltat = ts::dat(2008, 1, 17, 0.0).unwrap();
    assert!((deltat - 33.0).abs() < 1e-12, "iauDat d2");

    deltat = ts::dat(2017, 9, 1, 0.0).unwrap();
    assert!((deltat - 37.0).abs() < 1e-12, "iauDat d3");
}