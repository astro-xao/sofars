/// In the tangent plane projection, given the rectangular coordinates
/// of a star and its direction cosines, determine the direction
/// cosines of the tangent point.
///
/// Status:  support function.
///
/// Given:
///    xi,eta   f64    rectangular coordinates of star image (Note 2)
///    v        [f64; 3] star's direction cosines (Note 3)
///
/// Returned:
///    v01      [f64; 3] tangent point's direction cosines, Solution 1
///    v02      [f64; 3] tangent point's direction cosines, Solution 2
///    count    i32      number of solutions:
///                      0 = no solutions returned (Note 4)
///                      1 = only the first solution is useful (Note 5)
///                      2 = both solutions are useful (Note 5)
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
/// 3) The vector v must be of unit length or the result will be wrong.
///
/// 4) Cases where there is no solution can arise only near the poles.
///    For example, it is clearly impossible for a star at the pole
///    itself to have a non-zero xi value, and hence it is meaningless
///    to ask where the tangent point would have to be.
///
/// 5) Also near the poles, cases can arise where there are two useful
///    solutions.  The return value indicates whether the second of the
///    two solutions returned is useful;  1 indicates only one useful
///    solution, the usual case.
///
/// 6) The basis of the algorithm is to solve the spherical triangle
///    PSC, where P is the north celestial pole, S is the star and C is
///    the tangent point.  Calling the celestial spherical coordinates
///    of the star and tangent point (a,b) and (a0,b0) respectively, and
///    writing rho^2 = (xi^2+eta^2) and r^2 = (1+rho^2), and
///    transforming the vector v into (a,b) in the normal way, side c is
///    then (pi/2-b), side p is sqrt(xi^2+eta^2) and side s (to be
///    found) is (pi/2-b0), while angle C is given by sin(C) = xi/rho
///    and cos(C) = eta/rho;  angle P (to be found) is (a-a0).  After
///    solving the spherical triangle, the result (a0,b0) can be
///    expressed in vector form as v0.
///
/// 7) This function is a member of the following set:
///
///        spherical      vector         solve for
///
///        tpxes         tpxev          xi,eta
///        tpsts         tpstv           star
///        tpors       > tporv <        origin
///
/// References:
///
///    Calabretta M.R. & Greisen, E.W., 2002, "Representations of
///    celestial coordinates in FITS", Astron.Astrophys. 395, 1077
///
///    Green, R.M., "Spherical Astronomy", Cambridge University Press,
///    1987, Chapter 13.
pub fn tporv(xi: f64, eta: f64, v: [f64; 3]) -> ([f64; 3], [f64; 3], i32) {
    let x = v[0];
    let y = v[1];
    let z = v[2];
    let rxy2 = x * x + y * y;
    let xi2 = xi * xi;
    let eta2p1 = eta * eta + 1.0;
    let r = (xi2 + eta2p1).sqrt();
    let rsb = r * z;
    let rcb = r * (x * x + y * y).sqrt();
    let w2 = rcb * rcb - xi2;

    if w2 > 0.0 {
        let mut v01 = [0.0; 3];
        let mut v02 = [0.0; 3];

        let w = w2.sqrt();
        let c1 = (rsb * eta + w) / (eta2p1 * (rxy2 * (w2 + xi2)).sqrt());
        v01[0] = c1 * (x * w + y * xi);
        v01[1] = c1 * (y * w - x * xi);
        v01[2] = (rsb - eta * w) / eta2p1;

        let w_neg = -w;
        let c2 = (rsb * eta + w_neg) / (eta2p1 * (rxy2 * (w2 + xi2)).sqrt());
        v02[0] = c2 * (x * w_neg + y * xi);
        v02[1] = c2 * (y * w_neg - x * xi);
        v02[2] = (rsb - eta * w_neg) / eta2p1;

        let count = if rsb.abs() < 1.0 { 1 } else { 2 };
        (v01, v02, count)
    } else {
        ([0.0; 3], [0.0; 3], 0)
    }
}
