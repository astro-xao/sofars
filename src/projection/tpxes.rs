/// In the tangent plane projection, given celestial spherical
/// coordinates for a star and the tangent point, solve for the star's
/// rectangular coordinates in the tangent plane.
///
/// Status:  support function.
///
/// Given:
///    a,b       f64  star's spherical coordinates
///    a0,b0     f64  tangent point's spherical coordinates
///
/// Returned:
///    xi,eta    f64  rectangular coordinates of star image (Note 2)
///    status    i32  status:  0 = OK
///                          1 = star too far from axis
///                          2 = antistar on tangent plane
///                          3 = antistar too far from axis
///
/// Notes:
///
/// 1) The tangent plane projection is also called the "gnomonic
///    projection" and the "central projection".
///
/// 2) The eta axis points due north in the adopted coordinate system.
///    If the spherical coordinates are observed (RA,Dec), the tangent
///    plane coordinates (xi,eta) are conventionally called the
///    "standard coordinates".  For right-handed spherical coordinates,
///    (xi,eta) are also right-handed.  The units of (xi,eta) are,
///    effectively, radians at the tangent point.
///
/// 3) All angular arguments are in radians.
///
/// 4) This function is a member of the following set:
///
///        spherical      vector         solve for
///
///      > tpxes <       tpxev          xi,eta
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
pub fn tpxes(a: f64, b: f64, a0: f64, b0: f64) -> (f64, f64, i32) {
    const TINY: f64 = 1e-6;

    /* Functions of the spherical coordinates. */
    let sb0 = b0.sin();
    let sb = b.sin();
    let cb0 = b0.cos();
    let cb = b.cos();
    let da = a - a0;
    let sda = da.sin();
    let cda = da.cos();

    /* Reciprocal of star vector length to tangent plane. */
    let mut d = sb * sb0 + cb * cb0 * cda;

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
    let xi = cb * sda / d;
    let eta = (sb * cb0 - cb * sb0 * cda) / d;

    (xi, eta, status)
}
