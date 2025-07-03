use sofars::ts;

#[test]
fn test_d2dtf() {
    let (iy, im, id, ihmsf) = ts::d2dtf("UTC", 5, 2400000.5, 49533.99999).unwrap();

    assert_eq!(iy, 1994, "d2dtf: y");
    assert_eq!(im, 6, "d2dtf: mo");
    assert_eq!(id, 30, "d2dtf: d");
    assert_eq!(ihmsf[0], 23, "d2dtf: h");
    assert_eq!(ihmsf[1], 59, "d2dtf: m");
    assert_eq!(ihmsf[2], 60, "d2dtf: s");
    assert_eq!(ihmsf[3], 13599, "d2dtf: f");
}

#[test]
fn test_dat() {
    let mut deltat: f64;

    deltat = ts::dat(2003, 6, 1, 0.0).unwrap();
    assert!((deltat - 32.0).abs() < 1e-12, "dat: d1");

    deltat = ts::dat(2008, 1, 17, 0.0).unwrap();
    assert!((deltat - 33.0).abs() < 1e-12, "dat: d2");

    deltat = ts::dat(2017, 9, 1, 0.0).unwrap();
    assert!((deltat - 37.0).abs() < 1e-12, "dat: d3");
}

#[test]
fn test_ut1tt() {
    let (t1, t2) = ts::ut1tt(2453750.5, 0.892104561, 64.8499).unwrap();

    assert!((t1 - 2453750.5).abs() < 1e-6, "ut1tt: t1");
    assert!((t2 - 0.8928551385462962963).abs() < 1e-12, "ut1tt: t2");
}
