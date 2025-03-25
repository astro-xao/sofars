use sofars::erst::{
    ee00, ee00a, ee00b, ee06a, eect00, eqeq94, 
    gmst00, gmst06, gmst82, gst00a, gst00b, gst06, gst06a, gst94,
};

#[test]
fn test_ee00() {
    let epsa = 0.4090789763356509900;
    let dpsi = -0.9630909107115582393e-5;

    let ee = ee00(2400000.5, 53736.0, epsa, dpsi);

    assert!((ee - (-0.8834193235367965479e-5)).abs() < 1e-18, "ee00");
}

#[test]
fn test_ee00a() {
    let ee = ee00a(2400000.5, 53736.0);

    assert!((ee - (-0.8834192459222588227e-5)).abs() < 1e-18, "ee00a");
}

#[test]
fn test_ee00b() {
    let ee = ee00b(2400000.5, 53736.0);

    assert!((ee - (-0.8835700060003032831e-5)).abs() < 1e-18, "ee00b");
}

#[test]
fn test_ee06a() {
    let ee = ee06a(2400000.5, 53736.0);

    assert!((ee - (-0.8834195072043790156e-5)).abs() < 1e-15, "ee06a");
}

#[test]
fn test_eect00() {
    let eect = eect00(2400000.5, 53736.0);

    assert!((eect - 0.2046085004885125264e-8).abs() < 1e-20, "eect00");
}

#[test]
fn test_eqeq94() {
    let eqeq = eqeq94(2400000.5, 41234.0);

    assert!((eqeq - 0.5357758254609256894e-4).abs() < 1e-17, "eqeq94");
}

#[test]
fn test_gmst00() {
    let theta = gmst00(2400000.5, 53736.0, 2400000.5, 53736.0);

    assert!((theta - 1.754174972210740592).abs() < 1e-12, "gmst00");
}

#[test]
fn test_gmst06() {
    let theta = gmst06(2400000.5, 53736.0, 2400000.5, 53736.0);

    assert!((theta - 1.754174971870091203).abs() < 1e-12, "gmst06");
}

#[test]
fn test_gmst82() {
    let theta = gmst82(2400000.5, 53736.0);

    assert!((theta - 1.754174981860675096).abs() < 1e-12, "gmst82");
}

#[test]
fn test_gst00a() {
    let theta = gst00a(2400000.5, 53736.0, 2400000.5, 53736.0);

    assert!((theta - 1.754166138018281369).abs() < 1e-12, "gst00a");
}

#[test]
fn test_gst00b() {
    let theta = gst00b(2400000.5, 53736.0);

    assert!((theta - 1.754166136510680589).abs() < 1e-12, "gst00b");
}

#[test]
fn test_gst06() {
    let rnpb = [
        [0.9999989440476103608, -0.1332881761240011518e-2, -0.5790767434730085097e-3],
        [0.1332858254308954453e-2, 0.9999991109044505944, -0.4097782710401555759e-4],
        [0.5791308472168153320e-3, 0.4020595661593994396e-4, 0.9999998314954572365],
    ];

    let theta = gst06(2400000.5, 53736.0, 2400000.5, 53736.0, &rnpb);

    assert!((theta - 1.754166138018167568).abs() < 1e-12, "gst06");
}

#[test]
fn test_gst06a() {
    let theta = gst06a(2400000.5, 53736.0, 2400000.5, 53736.0);

    assert!((theta - 1.754166137675019159).abs() < 1e-12, "gst06a");
}

#[test]
fn test_gst94() {
    let theta = gst94(2400000.5, 53736.0);

    assert!((theta - 1.754166136020645203).abs() < 1e-12, "gst94");
}