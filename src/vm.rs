use std::ops::Rem;
use crate::consts::{self, D2PI, DAS2R, DPI};

pub fn a2af(ndp: i32, angle: f64) -> (char, [i32; 4]) {
    d2tf(ndp, angle * 15.0 / consts::D2PI)
}

pub fn af2a(s: char, ideg: i32, iamin: i32, asec: f64) -> Result<f64, i32> {
    let rad: f64;
    rad = match s {
        '-' => -1.0,
        _ => 1.0,
    } * (60.0 * ( 60.0 * ideg as f64 + iamin as f64) + asec as f64) * DAS2R;
    
    /* Validate arguments and return status. */
    if ideg < 0 || ideg > 359 {
        return Err(1);
    }
    if iamin < 0 || iamin > 59 {
        return Err(2);
    }
    if asec < 0.0 || asec >= 60.0 {
        return Err(3);
    }

    Ok(rad)
}

pub fn anp(a: f64) -> f64 {
    let mut w = a.rem(D2PI);
    if w < 0.0 {
        w += D2PI;
    }
    w
}

pub fn anpm(a: f64) -> f64 {
    let mut w = a.rem(D2PI);
    if w.abs() >= DPI {
        w -= consts::D2PI;
    }
    w
}

pub fn d2tf(ndp: i32, days: f64) -> (char, [i32; 4]) {
    // let mut ihmsf = vec![0; 4];
    let mut nrs;
    let (mut a, mut rs, rm, rh, w, ah, am, a_s, af);

    // Handle sign.
    let sign = if days >= 0.0 { '+' } else { '-' };

    // Interval in seconds.
    a = consts::DAYSEC * days.abs();

    // Pre-round if resolution coarser than 1s (then pretend ndp=1).
    if ndp < 0 {
        nrs = 1;
        for i in 1..=-ndp {
            nrs *= if i == 2 || i == 4 { 6 } else { 10 };
        }
        rs = nrs as f64;
        w = a / rs;
        a = rs * w.round();
    }

    // Express the unit of each field in resolution units.
    nrs = 1;
    for _ in 1..=ndp {
        nrs *= 10;
    }
    rs = nrs as f64;
    rm = rs * 60.0;
    rh = rm * 60.0;

    // Round the interval and express in resolution units.
    a = (rs * a).round();

    // Break into fields.
    ah = (a / rh).floor();
    a -= ah * rh;
    am = (a / rm).floor();
    a -= am * rm;
    a_s = (a / rs).floor();
    af = a - a_s * rs;

    // Return results.
    (sign, [ah as i32, am as i32, a_s as i32, af as i32])
}

pub fn tf2a() {}

pub fn tf2d() {}

pub fn rx(phi: f64, r: &mut [[f64; 3]; 3]) {
    let s = phi.sin();
    let c = phi.cos();

    let a10 = c * r[1][0] + s * r[2][0];
    let a11 = c * r[1][1] + s * r[2][1];
    let a12 = c * r[1][2] + s * r[2][2];
    let a20 = -s * r[1][0] + c * r[2][0];
    let a21 = -s * r[1][1] + c * r[2][1];
    let a22 = -s * r[1][2] + c * r[2][2];

    r[1][0] = a10;
    r[1][1] = a11;
    r[1][2] = a12;
    r[2][0] = a20;
    r[2][1] = a21;
    r[2][2] = a22;
}

pub fn ry(theta: f64, r: &mut [[f64; 3]; 3]) {
    let s = theta.sin();
    let c = theta.cos();

    let a00 = c * r[0][0] - s * r[2][0];
    let a01 = c * r[0][1] - s * r[2][1];
    let a02 = c * r[0][2] - s * r[2][2];
    let a20 = s * r[0][0] + c * r[2][0];
    let a21 = s * r[0][1] + c * r[2][1];
    let a22 = s * r[0][2] + c * r[2][2];

    r[0][0] = a00;
    r[0][1] = a01;
    r[0][2] = a02;
    r[2][0] = a20;
    r[2][1] = a21;
    r[2][2] = a22;
}

pub fn rz(psi: f64, r: &mut [[f64; 3]; 3]) {
    let s = psi.sin();
    let c = psi.cos();

    let a00 = c * r[0][0] + s * r[1][0];
    let a01 = c * r[0][1] + s * r[1][1];
    let a02 = c * r[0][2] + s * r[1][2];
    let a10 = -s * r[0][0] + c * r[1][0];
    let a11 = -s * r[0][1] + c * r[1][1];
    let a12 = -s * r[0][2] + c * r[1][2];

    r[0][0] = a00;
    r[0][1] = a01;
    r[0][2] = a02;
    r[1][0] = a10;
    r[1][1] = a11;
    r[1][2] = a12;
}

pub fn cp(p: &[f64; 3], c: &mut [f64; 3]) {
    c[0] = p[0];
    c[1] = p[1];
    c[2] = p[2];
}

pub fn cpv() {}

pub fn cr(r: &[[f64; 3]; 3], c: &mut [[f64; 3]; 3]) {
    cp(&r[0], &mut c[0]);
    cp(&r[1], &mut c[1]);
    cp(&r[2], &mut c[2]);
}

pub fn p2pv() {}

pub fn pv2p() {}

pub fn ir(r: &mut [[f64; 3]; 3]) {
    r[0][0] = 1.0;
    r[0][1] = 0.0;
    r[0][2] = 0.0;
    r[1][0] = 0.0;
    r[1][1] = 1.0;
    r[1][2] = 0.0;
    r[2][0] = 0.0;
    r[2][1] = 0.0;
    r[2][2] = 1.0;
}

pub fn zp(p: &mut [f64; 3]) {
    p[0] = 0.0;
    p[1] = 0.0;
    p[2] = 0.0;
}

pub fn zpv() {}

pub fn zr() {}

pub fn rxr() {}

pub fn tr(r: &[[f64; 3]; 3], rt: &mut [[f64; 3]; 3]) {
    let wm  = &mut [[0.0; 3]; 3];
    for i in 0..3 {
        for j in 0..3 {
            wm[i][j] = r[j][i];
        }
    }
    cr(wm, rt);
}

pub fn rxp(r: &[[f64; 3]; 3], p: &[f64; 3], rp: &mut [f64; 3]) {
    let wrp = &mut [0.0; 3];

    // Matrix r * vector p.
    for j in 0..3 {
        let mut w = 0.0;
        for i in 0..3 {
            w += r[j][i] * p[i];
        }
        wrp[j] = w;
    }

    /* Return the result. */
    cp(wrp, rp);
}

pub fn rxpv(r: &[[f64; 3]; 3], pv: &[[f64; 3]; 2], rpv: &mut [[f64; 3]; 2]) {
    rxp(r, &pv[0], &mut rpv[0]);
    rxp(r, &pv[1], &mut rpv[1]);
}

pub fn trxp(r: &[[f64; 3]; 3], p: &[f64; 3], trp: &mut [f64; 3]) {
    let tr_ = &mut [[0.0; 3]; 3];
    tr(r, tr_);
    rxp(tr_, p, trp);
}

pub fn trxpv(r: &[[f64; 3]; 3], pv: &[[f64; 3]; 2], trpv: &mut [[f64; 3]; 2]) {
    let tr_= &mut [[0.0; 3]; 3];

    /* Transpose of matrix r. */
    tr(r, tr_);

    /* Matrix tr * vector pv -> vector trpv. */
    rxpv(tr_, pv, trpv);
}

pub fn rm2v() {}

pub fn rv2m() {}

pub fn pap() {}

pub fn pas() {}

pub fn sepp() {}

pub fn seps() {}

pub fn c2s(p: &[f64; 3]) -> (f64, f64) {
    let x = p[0];
    let y = p[1];
    let z = p[2];
    let d2 = x * x + y * y;

    let theta = if d2 == 0.0 { 0.0 } else { y.atan2(x) };
    let phi = if z == 0.0 { 0.0 } else { z.atan2(d2.sqrt()) };

    (theta, phi)
}

pub fn p2s() {}

pub fn pv2s() {}

pub fn s2c(theta: f64, phi: f64) -> [f64; 3] {
    let cp = phi.cos();
    [theta.cos() * cp, theta.sin() * cp, phi.sin()]
}

pub fn s2p() {}

pub fn s2pv() {}

pub fn pdp(a: &[f64; 3], b: &[f64; 3]) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}

pub fn pm(p: [f64; 3]) -> f64 {
    (p[0] * p[0] + p[1] * p[1] + p[2] * p[2]).sqrt()
}

pub fn pmp() {}

pub fn pn(p: &[f64; 3]) -> (f64, [f64; 3]) {
    let r = (p[0] * p[0] + p[1] * p[1] + p[2] * p[2]).sqrt();
    let u = if r != 0.0 {
        [p[0] / r, p[1] / r, p[2] / r]
    } else {
        [0.0, 0.0, 0.0]
    };
    (r, u)
}

pub fn ppp() {}

pub fn ppsp() {}

pub fn pvdpv() {}

pub fn pvm() {}

pub fn pvmpv() {}

pub fn pvppv() {}

pub fn pvu() {}

pub fn pvup() {}

pub fn pvxpv() {}

pub fn pxp(a: &[f64; 3], b: &[f64; 3]) -> [f64; 3] {
    let mut axb: [f64; 3] = [0.0; 3];

    axb[0] = a[1] * b[2] - a[2] * b[1];
    axb[1] = a[2] * b[0] - a[0] * b[2];
    axb[2] = a[0] * b[1] - a[1] * b[0];

    axb
}

pub fn s2xpv() {}

pub fn sxp(s: f64, p: &[f64; 3]) -> [f64; 3] {
    [s * p[0], s * p[1], s * p[2]]
}

pub fn sxpv() {}
