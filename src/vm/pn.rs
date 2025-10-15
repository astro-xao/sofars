/// Convert a p-vector into modulus and unit vector.
pub fn pn(p: &[f64; 3]) -> (f64, [f64; 3]) {
    let r = (p[0] * p[0] + p[1] * p[1] + p[2] * p[2]).sqrt();
    let u = if r != 0.0 {
        [p[0] / r, p[1] / r, p[2] / r]
    } else {
        [0.0, 0.0, 0.0]
    };
    (r, u)
}
