use crate::vm::{cr, rxr, rz};

///  Assemble the celestial to terrestrial matrix from equinox-based
///  components (the celestial-to-true matrix, the Greenwich Apparent
///  Sidereal Time and the polar motion matrix).
///
///  Given:
///     rbpn   [[f64; 3]; 3]  celestial-to-true matrix
///     gst    f64            Greenwich (apparent) Sidereal Time (radians)
///     rpom   [[f64; 3]; 3]  polar-motion matrix
///
///  Returned (function value):
///                 [[f64; 3]; 3]  celestial-to-terrestrial matrix (Note 2)
///
///  Notes:
///
///  1) This function constructs the rotation matrix that transforms
///     vectors in the celestial system into vectors in the terrestrial
///     system.  It does so starting from precomputed components, namely
///     the matrix which rotates from celestial coordinates to the
///     true equator and equinox of date, the Greenwich Apparent Sidereal
///     Time and the polar motion matrix.  One use of the present function
///     is when generating a series of celestial-to-terrestrial matrices
///     where only the Sidereal Time changes, avoiding the considerable
///     overhead of recomputing the precession-nutation more often than
///     necessary to achieve given accuracy objectives.
///
///  2) The relationship between the arguments is as follows:
///
///        [TRS] = rpom * R_3(gst) * rbpn * [CRS]
///
///              = rc2t * [CRS]
///
///     where [CRS] is a vector in the Geocentric Celestial Reference
///     System and [TRS] is a vector in the International Terrestrial
///     Reference System (see IERS Conventions 2003).
///
///  Called:
///     iauCr        copy r-matrix
///     iauRz        rotate around Z-axis
///     iauRxr       product of two r-matrices
///
///  Reference:
///
///     McCarthy, D. D., Petit, G. (eds.), IERS Conventions (2003),
///     IERS Technical Note No. 32, BKG (2004)
pub fn c2teqx(rbpn: &[[f64; 3]; 3], gst: f64, rpom: &[[f64; 3]; 3]) -> [[f64; 3]; 3] {
    let mut r = [[0.0; 3]; 3];
    let mut rc2t = [[0.0; 3]; 3];

    /* Construct the matrix. */
    cr(rbpn, &mut r);
    rz(gst, &mut r);
    rxr(rpom, &r, &mut rc2t);
    rc2t
}
