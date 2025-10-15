/// Convert spherical coordinates to Cartesian.
pub fn s2c(theta: f64, phi: f64) -> [f64; 3] {
    let cp = phi.cos();
    [theta.cos() * cp, theta.sin() * cp, phi.sin()]
}
