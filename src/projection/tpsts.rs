use crate::vm::anp;

/// In the tangent plane projection, given the star's rectangular
/// coordinates and the spherical coordinates of the tangent point,
/// solve for the spherical coordinates of the star.
///
/// Status:  support function.
///
/// Given:
///    xi,eta    f64  rectangular coordinates of star image (Note 2)
///    a0,b0     f64  tangent point's spherical coordinates
///
/// Returned:
///    a,b       f64  star's spherical coordinates
///
/// Notes:
///
/// 1) The tangent plane projection is also called the "gnomonic
///    projection" and the "central projection".
///
/// 2) The eta axis points due north in the adopted coordinate system.
///    If the spherical coordinates are observed (RA,Dec), the tangent
///    plane coordinates (xi,eta) are conventionally called the
///    "standard coordinates".  If the spherical coordinates are with
///    respect to a right-handed triad, (xi,eta) are also right-handed.
///    The units of (xi,eta) are, effectively, radians at the tangent
///    point.
///
/// 3) All angular arguments are in radians.
///
/// 4) This function is a member of the following set:
///
///        spherical      vector         solve for
///
///        tpxes         tpxev          xi,eta
///      > tpsts <       tpstv           star
///        tpors         tporv          origin
///
/// References:
///
///    Calabretta M.R. & Greisen, E.W., 2002, "Representations of
///    celestial coordinates in FITS", Astron.Astrophys. 395, 1077
///
///    Green, R.M., "Spherical Astronomy", Cambridge University Press,
///    1987, Chapter 13.
pub fn tpsts(xi: f64, eta: f64, a0: f64, b0: f64) -> (f64, f64) {
    let sb0 = b0.sin();
    let cb0 = b0.cos();
    let d = cb0 - eta * sb0;
    let a = anp(xi.atan2(d) + a0);
    let b = (sb0 + eta * cb0).atan2((xi * xi + d * d).sqrt());
    (a, b)
}
