use sofars::erst::{ee00, ee00a, ee00b};

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