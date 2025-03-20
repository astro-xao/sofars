use sofars::vm::*;

#[test]
fn test_a2af() {
    let result = a2af(4, 2.345);
    let (sign, dmsf) = (result.0, result.1);
    assert_eq!(sign, '+');
    assert_eq!(dmsf[0], 134);
    assert_eq!(dmsf[1], 21);
    assert_eq!(dmsf[2], 30);
    assert_eq!(dmsf[3], 9706);
}

#[test]
fn test_af2a() {
    let a = af2a('-', 45, 13, 27.2).unwrap();
    assert!((a - (-0.7893115794313644842)).abs() < 1e-12, "af2a a");
}

#[test]
fn test_tr() {
    let r = &[
        [2.0, 3.0, 2.0],
        [3.0, 2.0, 3.0],
        [3.0, 4.0, 5.0],
    ];

    let rt = &mut [[0.0; 3]; 3];

    tr(r, rt);

    assert!((rt[0][0] - 2.0).abs() < 1e-12, "tr 11");
    assert!((rt[0][1] - 3.0).abs() < 1e-12, "tr 12");
    assert!((rt[0][2] - 3.0).abs() < 1e-12, "tr 13");

    assert!((rt[1][0] - 3.0).abs() < 1e-12, "tr 21");
    assert!((rt[1][1] - 2.0).abs() < 1e-12, "tr 22");
    assert!((rt[1][2] - 4.0).abs() < 1e-12, "tr 23");

    assert!((rt[2][0] - 2.0).abs() < 1e-12, "tr 31");
    assert!((rt[2][1] - 3.0).abs() < 1e-12, "tr 32");
    assert!((rt[2][2] - 5.0).abs() < 1e-12, "tr 33");
}

#[test]
fn test_rxp() {
    let r = [
        [2.0, 3.0, 2.0],
        [3.0, 2.0, 3.0],
        [3.0, 4.0, 5.0],
    ];

    let p = [0.2, 1.5, 0.1];

    let mut rp = [0.0; 3];

    rxp(&r, &p, &mut rp);

    assert!((rp[0] - 5.1).abs() < 1e-12, "rxp 1");
    assert!((rp[1] - 3.9).abs() < 1e-12, "rxp 2");
    assert!((rp[2] - 7.1).abs() < 1e-12, "rxp 3");
}

#[test]
fn test_rxpv() {
    let r = [
        [2.0, 3.0, 2.0],
        [3.0, 2.0, 3.0],
        [3.0, 4.0, 5.0],
    ];
    let pv = [
        [0.2, 1.5, 0.1],
        [1.5, 0.2, 0.1],
    ];
    let mut rpv = [[0.0; 3]; 2];

    rxpv(&r, &pv, &mut rpv);

    assert!((rpv[0][0] - 5.1).abs() < 1e-12);
    assert!((rpv[1][0] - 3.8).abs() < 1e-12);

    assert!((rpv[0][1] - 3.9).abs() < 1e-12);
    assert!((rpv[1][1] - 5.2).abs() < 1e-12);

    assert!((rpv[0][2] - 7.1).abs() < 1e-12);
    assert!((rpv[1][2] - 5.8).abs() < 1e-12);
}

#[test]
fn test_trxpv() {
    let r = [
        [2.0, 3.0, 2.0],
        [3.0, 2.0, 3.0],
        [3.0, 4.0, 5.0],
    ];

    let pv = [
        [0.2, 1.5, 0.1],
        [1.5, 0.2, 0.1],
    ];

    let mut trpv = [[0.0; 3]; 2];

    trxpv(&r, &pv, &mut trpv);

    assert!((trpv[0][0] - 5.2).abs() < 1e-12, "trxpv p1");
    assert!((trpv[0][1] - 4.0).abs() < 1e-12, "trxpv p1");
    assert!((trpv[0][2] - 5.4).abs() < 1e-12, "trxpv p1");

    assert!((trpv[1][0] - 3.9).abs() < 1e-12, "trxpv v1");
    assert!((trpv[1][1] - 5.3).abs() < 1e-12, "trxpv v2");
    assert!((trpv[1][2] - 4.1).abs() < 1e-12, "trxpv v3");
}