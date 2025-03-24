use sofars::pnp::{c2ixys, nut00a, nut00b, nut06a, pnm00a, pnm06a, s06};

#[test]
fn test_c2ixys() {
    let x = 0.5791308486706011000e-3;
    let y = 0.4020579816732961219e-4;
    let s = -0.1220040848472271978e-7;

    let rc2i = &mut [[0.0; 3]; 3];
    c2ixys(x, y, s, rc2i);

    assert!((rc2i[0][0] - 0.9999998323037157138).abs() < 1e-12, "c2ixys 11");
    assert!((rc2i[0][1] - 0.5581984869168499149e-9).abs() < 1e-12, "c2ixys 12");
    assert!((rc2i[0][2] - (-0.5791308491611282180e-3)).abs() < 1e-12, "c2ixys 13");

    assert!((rc2i[1][0] - (-0.2384261642670440317e-7)).abs() < 1e-12, "c2ixys 21");
    assert!((rc2i[1][1] - 0.9999999991917468964).abs() < 1e-12, "c2ixys 22");
    assert!((rc2i[1][2] - (-0.4020579110169668931e-4)).abs() < 1e-12, "c2ixys 23");

    assert!((rc2i[2][0] - 0.5791308486706011000e-3).abs() < 1e-12, "c2ixys 31");
    assert!((rc2i[2][1] - 0.4020579816732961219e-4).abs() < 1e-12, "c2ixys 32");
    assert!((rc2i[2][2] - 0.9999998314954627590).abs() < 1e-12, "c2ixys 33");
}

#[test]
fn test_nut06a() {
    let (dpsi, deps) = nut06a(2400000.5, 53736.0);

    assert!((dpsi - (-0.9630912025820308797e-5)).abs() < 1e-13, "nut06a dpsi");
    assert!((deps - 0.4063238496887249798e-4).abs() < 1e-13, "nut06a deps");
}

#[test]
fn test_nut00a() {
    let (dpsi, deps) = nut00a(2400000.5, 53736.0);

    assert!((dpsi - (-0.9630909107115518431e-5)).abs() < 1e-13, "nut00a dpsi");
    assert!((deps - 0.4063239174001678710e-4).abs() < 1e-13, "nut00a deps");
}

#[test]
fn test_nut00b() {
    let (dpsi, deps) = nut00b(2400000.5, 53736.0);

    assert!((dpsi - (-0.9632552291148362783e-5)).abs() < 1e-13, "nut00b dpsi");
    assert!((deps - 0.4063197106621159367e-4).abs() < 1e-13, "nut00b deps");
}

#[test]
fn test_pnm00a() {
    let rbpn = &mut [[0.0; 3]; 3];
    pnm00a(2400000.5, 50123.9999, rbpn);

    assert!((rbpn[0][0] - 0.9999995832793134257).abs() < 1e-12, "pnm00a 11");
    assert!((rbpn[0][1] - 0.8372384254137809439e-3).abs() < 1e-14, "pnm00a 12");
    assert!((rbpn[0][2] - 0.3639684306407150645e-3).abs() < 1e-14, "pnm00a 13");

    assert!((rbpn[1][0] - -0.8372535226570394543e-3).abs() < 1e-14, "pnm00a 21");
    assert!((rbpn[1][1] - 0.9999996486491582471).abs() < 1e-12, "pnm00a 22");
    assert!((rbpn[1][2] - 0.4132915262664072381e-4).abs() < 1e-14, "pnm00a 23");

    assert!((rbpn[2][0] - -0.3639337004054317729e-3).abs() < 1e-14, "pnm00a 31");
    assert!((rbpn[2][1] - -0.4163386925461775873e-4).abs() < 1e-14, "pnm00a 32");
    assert!((rbpn[2][2] - 0.9999999329094390695).abs() < 1e-12, "pnm00a 33");
}

#[test]
fn test_pnm06a() {
    let rbpn = pnm06a(2400000.5, 50123.9999);

    assert!((rbpn[0][0] - 0.9999995832794205484).abs() < 1e-12, "pnm06a 11");
    assert!((rbpn[0][1] - 0.8372382772630962111e-3).abs() < 1e-14, "pnm06a 12");
    assert!((rbpn[0][2] - 0.3639684771140623099e-3).abs() < 1e-14, "pnm06a 13");

    assert!((rbpn[1][0] - -0.8372533744743683605e-3).abs() < 1e-14, "pnm06a 21");
    assert!((rbpn[1][1] - 0.9999996486492861646).abs() < 1e-12, "pnm06a 22");
    assert!((rbpn[1][2] - 0.4132905944611019498e-4).abs() < 1e-14, "pnm06a 23");

    assert!((rbpn[2][0] - -0.3639337469629464969e-3).abs() < 1e-14, "pnm06a 31");
    assert!((rbpn[2][1] - -0.4163377605910663999e-4).abs() < 1e-14, "pnm06a 32");
    assert!((rbpn[2][2] - 0.9999999329094260057).abs() < 1e-12, "pnm06a 33");
}

#[test]
fn test_s06() {
    let x = 0.5791308486706011000e-3;
    let y = 0.4020579816732961219e-4;

    let s = s06(2400000.5, 53736.0, x, y);

    assert!((s - -0.1220032213076463117e-7).abs() < 1e-18, "s06");
}