use crate::erst::era00;
use crate::pnp::{c2i00b, c2tcio, pom00};

///  Form the celestial to terrestrial matrix given the date, the UT1 and
///  the polar motion, using the IAU 2000B precession-nutation model.
///
///  Given:
///     tta,ttb  f64          TT as a 2-part Julian Date (Note 1)
///     uta,utb  f64          UT1 as a 2-part Julian Date (Note 1)
///     xp,yp    f64          coordinates of the pole (radians, Note 2)
///
///  Returned (function value):
///                 [[f64; 3]; 3]   celestial-to-terrestrial matrix (Note 3)
///
///  Notes:
///
///  1) The TT and UT1 dates tta+ttb and uta+utb are Julian Dates,
///     apportioned in any convenient way between the arguments uta and
///     utb.  For example, JD(UT1)=2450123.7 could be expressed in any of
///     these ways, among others:
///
///             uta            utb
///
///         2450123.7           0.0       (JD method)
///         2451545.0       -1421.3       (J2000 method)
///         2400000.5       50123.2       (MJD method)
///         2450123.5           0.2       (date & time method)
///
///     The JD method is the most natural and convenient to use in
///     cases where the loss of several decimal digits of resolution is
///     acceptable.  The J2000 and MJD methods are good compromises
///     between resolution and convenience.  In the case of uta,utb, the
///     date & time method is best matched to the Earth rotation angle
///     algorithm used:  maximum precision is delivered when the uta
///     argument is for 0hrs UT1 on the day in question and the utb
///     argument lies in the range 0 to 1, or vice versa.
///
///  2) The arguments xp and yp are the coordinates (in radians) of the
///     Celestial Intermediate Pole with respect to the International
///     Terrestrial Reference System (see IERS Conventions 2003),
///     measured along the meridians 0 and 90 deg west respectively.
///
///  3) The matrix rc2t transforms from celestial to terrestrial
///     coordinates:
///
///        [TRS] = RPOM * R_3(ERA) * RC2I * [CRS]
///
///              = rc2t * [CRS]
///
///     where [CRS] is a vector in the Geocentric Celestial Reference
///     System and [TRS] is a vector in the International Terrestrial
///     Reference System (see IERS Conventions 2003), RC2I is the
///     celestial-to-intermediate matrix, ERA is the Earth rotation
///     angle and RPOM is the polar motion matrix.
///
///  4) The present function is faster, but slightly less accurate (about
///     1 mas), than the iauC2t00a function.
///
///  Called:
///     iauC2i00b    celestial-to-intermediate matrix, IAU 2000B
///     iauEra00     Earth rotation angle, IAU 2000
///     iauPom00     polar motion matrix
///     iauC2tcio    form CIO-based celestial-to-terrestrial matrix
///
///  Reference:
///
///     McCarthy, D. D., Petit, G. (eds.), IERS Conventions (2003),
///     IERS Technical Note No. 32, BKG (2004)
pub fn c2t00b(tta: f64, ttb: f64, uta: f64, utb: f64, xp: f64, yp: f64) -> [[f64; 3]; 3] {
    /* Form the celestial-to-intermediate matrix for this TT (IAU 2000B). */
    let rc2i = c2i00b(tta, ttb);

    /* Predict the Earth rotation angle for this UT1. */
    let era = era00(uta, utb);

    /* Form the polar motion matrix (neglecting s'). */
    let rpom = pom00(xp, yp, 0.0);

    /* Combine to form the celestial-to-terrestrial matrix. */
    c2tcio(&rc2i, era, &rpom)
}
