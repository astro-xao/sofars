pub fn pv2s(pv: &[[f64; 3]; 2]) -> (f64, f64, f64, f64, f64, f64) {
    let (theta, phi, r, td, pd, rd): (f64, f64, f64, f64, f64, f64);

    let (mut x, mut y, mut z): (f64, f64, f64);
    let (xd, yd, zd, mut rxy2, rxy): (f64, f64, f64, f64, f64);
    let (mut r2, rtrue, mut rw, xyp): (f64, f64, f64, f64);

    /* Components of position/velocity vector. */
    x = pv[0][0];
    y = pv[0][1];
    z = pv[0][2];
    xd = pv[1][0];
    yd = pv[1][1];
    zd = pv[1][2];

    /* Component of r in XY plane squared. */
    rxy2 = x * x + y * y;

    /* Modulus squared. */
    r2 = rxy2 + z * z;

    /* Modulus. */
    rtrue = r2.sqrt();

    /* If null vector, move the origin along the direction of movement. */
    rw = rtrue;
    if rtrue == 0.0 {
        x = xd;
        y = yd;
        z = zd;
        rxy2 = x * x + y * y;
        r2 = rxy2 + z * z;
        rw = r2.sqrt();
    }

    /* Position and velocity in spherical coordinates. */
    rxy = rxy2.sqrt();
    xyp = x * xd + y * yd;
    if rxy2 != 0.0 {
        theta = y.atan2(x);
        phi = z.atan2(rxy);
        td = (x * yd - y * xd) / rxy2;
        pd = (zd * rxy2 - z * xyp) / (r2 * rxy);
    } else {
        theta = 0.0;
        phi = if z != 0.0 { z.atan2(rxy) } else { 0.0 };
        td = 0.0;
        pd = 0.0;
    }
    r = rtrue;
    rd = if rw != 0.0 { (xyp + z * zd) / rw } else { 0.0 };

    (theta, phi, r, td, pd, rd)
}
