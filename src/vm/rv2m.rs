/// Form the r-matrix corresponding to a given r-vector.
///
/// Status:  vector/matrix support function.
///
/// Given:
///    w        [f64; 3]      rotation vector (Note 1)
///
/// Returned:
///    r        [[f64; 3]; 3] rotation matrix
///
/// Notes:
///
/// 1) A rotation matrix describes a rotation through some angle about
///    some arbitrary axis called the Euler axis.  The "rotation vector"
///    supplied to This function has the same direction as the Euler
///    axis, and its magnitude is the angle in radians.
///
/// 2) If w is null, the identity matrix is returned.
///
/// 3) The reference frame rotates clockwise as seen looking along the
///    rotation vector from the origin.
pub fn rv2m(w: &[f64; 3], r: &mut [[f64; 3]; 3]) {
    let x = w[0];
    let y = w[1];
    let z = w[2];
    let phi = (x * x + y * y + z * z).sqrt();
    let s = phi.sin();
    let c = phi.cos();
    let f = 1.0 - c;

    let (ux, uy, uz) = if phi > 0.0 {
        (x / phi, y / phi, z / phi)
    } else {
        (0.0, 0.0, 0.0)
    };

    r[0][0] = ux * ux * f + c;
    r[0][1] = ux * uy * f + uz * s;
    r[0][2] = ux * uz * f - uy * s;
    r[1][0] = uy * ux * f - uz * s;
    r[1][1] = uy * uy * f + c;
    r[1][2] = uy * uz * f + ux * s;
    r[2][0] = uz * ux * f + uy * s;
    r[2][1] = uz * uy * f - ux * s;
    r[2][2] = uz * uz * f + c;
}
