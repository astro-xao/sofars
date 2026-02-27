/// In the tangent plane projection, given celestial direction cosines
/// for a star and the tangent point, solve for the star's rectangular
/// coordinates in the tangent plane.
///
/// Status:  support function.
///
/// Given:
///    v         [f64; 3]  direction cosines of star (Note 4)
///    v0        [f64; 3]  direction cosines of tangent point (Note 4)
///
/// Returned:
///    xi,eta    f64     tangent plane coordinates of star
///    status    i32     status: 0 = OK
///                              1 = star too far from axis
///                              2 = antistar on tangent plane
///                              3 = antistar too far from axis
///
/// Notes:
///
/// 1) The tangent plane projection is also called the "gnomonic
///    projection" and the "central projection".
///
/// 2) The eta axis points due north in the adopted coordinate system.
///    If the direction cosines represent observed (RA,Dec), the tangent
///    plane coordinates (xi,eta) are conventionally called the
///    "standard coordinates".  If the direction cosines are with
///    respect to a right-handed triad, (xi,eta) are also right-handed.
///    The units of (xi,eta) are, effectively, radians at the tangent
///    point.
///
/// 3) The method used is to extend the star vector to the tangent
///    plane and then rotate the triad so that (x,y) becomes (xi,eta).
///    Writing (a,b) for the celestial spherical coordinates of the
///    star, the sequence of rotations is (a+pi/2) around the z-axis
///    followed by (pi/2-b) around the x-axis.
///
/// 4) If vector v0 is not of unit length, or if vector v is of zero
///    length, the results will be wrong.
///
/// 5) If v0 points at a pole, the returned (xi,eta) will be based on
///    the arbitrary assumption that the longitude coordinate of the
///    tangent point is zero.
///
/// 6) This function is a member of the following set:
///
///        spherical      vector         solve for
///
///        tpxes       > tpxev <       xi,eta
///        tpsts         tpstv           star
///        tpors         tporv          origin
///
/// References:
///
///    Calabretta M.R. & Greisen, E.W., 2002, "Representations of
///    celestial coordinates in FITS", Astron.Astrophys. 395, 1077
///
///    Green, R.M., "Spherical Astronomy", Cambridge University Press,
///    1987, Chapter 13.
pub fn tpxev(v: [f64; 3], v0: [f64; 3]) -> (f64, f64, i32) {
    const TINY: f64 = 1e-6;

    let x = v[0];
    let y = v[1];
    let z = v[2];
    let mut x0 = v0[0];
    let y0 = v0[1];
    let z0 = v0[2];

    /* Deal with polar case. */
    let r2 = x0 * x0 + y0 * y0;
    let mut r = r2.sqrt();
    if r == 0.0 {
        r = 1e-20;
        x0 = r;
    }

    /* Reciprocal of star vector length to tangent plane. */
    let w = x * x0 + y * y0;
    let mut d = w + z * z0;

    /* Check for error cases. */
    let status = if d > TINY {
        0
    } else if d >= 0.0 {
        d = TINY;
        1
    } else if d > -TINY {
        d = -TINY;
        2
    } else {
        3
    };

    /* Return the tangent plane coordinates (even in dubious cases). */
    let d_final = d * r;
    let xi = (y * x0 - x * y0) / d_final;
    let eta = (z * r2 - z0 * w) / d_final;

    (xi, eta, status)
}
