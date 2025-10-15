/// P-vector to spherical coordinates.
pub fn c2s(p: &[f64; 3]) -> (f64, f64) {
    let x = p[0];
    let y = p[1];
    let z = p[2];
    let d2 = x * x + y * y;

    let theta = if d2 == 0.0 { 0.0 } else { y.atan2(x) };
    let phi = if z == 0.0 { 0.0 } else { z.atan2(d2.sqrt()) };

    (theta, phi)
}
