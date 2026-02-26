use crate::vm::{cr, rxr, rz};

///  Assemble the celestial to terrestrial matrix from CIO-based
///  components (the celestial-to-intermediate matrix, the Earth Rotation
///  Angle and the polar motion matrix).
///
///  Given:
///     rc2i     [[f64; 3]; 3]    celestial-to-intermediate matrix
///     era      f64              Earth rotation angle (radians)
///     rpom     [[f64; 3]; 3]    polar-motion matrix
///
///  Returned (function value):
///                 [[f64; 3]; 3]    celestial-to-terrestrial matrix
///
///  Notes:
///
///  1) This function constructs the rotation matrix that transforms
///     vectors in the celestial system into vectors in the terrestrial
///     system.  It does so starting from precomputed components, namely
///     the matrix which rotates from celestial coordinates to the
///     intermediate frame, the Earth rotation angle and the polar motion
///     matrix.  One use of the present function is when generating a
///     series of celestial-to-terrestrial matrices where only the Earth
///     Rotation Angle changes, avoiding the considerable overhead of
///     recomputing the precession-nutation more often than necessary to
///     achieve given accuracy objectives.
///
///  2) The relationship between the arguments is as follows:
///
///        [TRS] = RPOM * R_3(ERA) * rc2i * [CRS]
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
///     McCarthy, D. D., Petit, G. (eds.), 2004, IERS Conventions (2003),
///     IERS Technical Note No. 32, BKG
pub fn c2tcio(rc2i: &[[f64; 3]; 3], era: f64, rpom: &[[f64; 3]; 3]) -> [[f64; 3]; 3] {
    let mut r = [[0.0; 3]; 3];
    let mut rc2t = [[0.0; 3]; 3];

    /* Construct the matrix. */
    cr(rc2i, &mut r);
    rz(era, &mut r);
    rxr(rpom, &r, &mut rc2t);
    rc2t
}
