/// In the tangent plane projection, given the star's rectangular
/// coordinates and the direction cosines of the tangent point, solve
/// for the direction cosines of the star.
///
/// Status:  support function.
///
/// Given:
///    xi,eta  f64     rectangular coordinates of star image (Note 2)
///    v0      [f64; 3]  tangent point's direction cosines
///
/// Returned:
///    v       [f64; 3]  star's direction cosines
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
/// 3) The method used is to complete the star vector in the (xi,eta)
///    based triad and normalize it, then rotate the triad to put the
///    tangent point at the pole with the x-axis aligned to zero
///    longitude.  Writing (a0,b0) for the celestial spherical
///    coordinates of the tangent point, the sequence of rotations is
///    (b-pi/2) around the x-axis followed by (-a-pi/2) around the
///    z-axis.
///
/// 4) If vector v0 is not of unit length, the returned vector v will
///    be wrong.
///
/// 5) If vector v0 points at a pole, the returned vector v will be
///    based on the arbitrary assumption that the longitude coordinate
///    of the tangent point is zero.
///
/// 6) This function is a member of the following set:
///
///        spherical      vector         solve for
///
///        tpxes         tpxev          xi,eta
///        tpsts       > tpstv <         star
///        tpors         tporv          origin
///
/// References:
///
///    Calabretta M.R. & Greisen, E.W., 2002, "Representations of
///    celestial coordinates in FITS", Astron.Astrophys. 395, 1077
///
///    Green, R.M., "Spherical Astronomy", Cambridge University Press,
///    1987, Chapter 13.
pub fn tpstv(xi: f64, eta: f64, v0: [f64; 3]) -> [f64; 3] {
    let mut x = v0[0];
    let y = v0[1];
    let z = v0[2];

    /* Deal with polar case. */
    let mut r = (x * x + y * y).sqrt();
    if r == 0.0 {
        r = 1e-20;
        x = r;
    }

    /* Star vector length to tangent plane. */
    let f = (1.0 + xi * xi + eta * eta).sqrt();

    /* Apply the transformation and normalize. */
    let v_x = (x - (xi * y + eta * x * z) / r) / f;
    let v_y = (y + (xi * x - eta * y * z) / r) / f;
    let v_z = (z + eta * r) / f;

    [v_x, v_y, v_z]
}
