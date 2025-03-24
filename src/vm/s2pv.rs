///  Convert position/velocity from spherical to Cartesian coordinates.
///
///  This function is part of the International Astronomical Union's
///  SOFA (Standards of Fundamental Astronomy) software collection.
///
///  Status:  vector/matrix support function.
///
///  Given:
///  ```
///     theta    double          longitude angle (radians)
///     phi      double          latitude angle (radians)
///     r        double          radial distance
///     td       double          rate of change of theta
///     pd       double          rate of change of phi
///     rd       double          rate of change of r
///  ```
///  Returned:
///  ```
///     pv       double[2][3]    pv-vector
///  ```
pub fn s2pv(theta: f64, phi: f64, r: f64, td: f64, pd: f64, rd: f64) -> [[f64; 3]; 2] {
    let mut pv = [[0.0; 3]; 2];
    
    let st = theta.sin();
    let ct = theta.cos();
    let sp = phi.sin();
    let cp = phi.cos();
    let rcp = r * cp;
    let x = rcp * ct;
    let y = rcp * st;
    let rpd = r * pd;
    let w = rpd * sp - cp * rd;

    pv[0][0] = x;
    pv[0][1] = y;
    pv[0][2] = r * sp;
    pv[1][0] = -y * td - w * ct;
    pv[1][1] = x * td - w * st;
    pv[1][2] = rpd * cp + sp * rd;

    pv
}