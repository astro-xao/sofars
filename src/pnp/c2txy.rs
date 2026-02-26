use crate::erst::era00;
use crate::pnp::{c2ixy, c2tcio, pom00, sp00};

///  Form the celestial to terrestrial matrix given the date, the UT1,
///  the CIP coordinates and the polar motion.  IAU 2000.
///
///  Given:
///     tta,ttb  f64          TT as a 2-part Julian Date (Note 1)
///     uta,utb  f64          UT1 as a 2-part Julian Date (Note 1)
///     x,y      f64          Celestial Intermediate Pole (Note 2)
///     xp,yp    f64          coordinates of the pole (radians, Note 3)
///
///  Returned (function value):
///                 [[f64; 3]; 3]   celestial-to-terrestrial matrix (Note 4)
///
///  Notes:
///
///  1) The TT and UT1 dates tta+ttb and uta+utb are Julian Dates,
///     apportioned in any convenient way between the arguments uta and
///     utb.  For example, JD(UT1)=2450123.7 could be expressed in any o
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
///  2) The Celestial Intermediate Pole coordinates are the x,y
///     components of the unit vector in the Geocentric Celestial
///     Reference System.
///
///  3) The arguments xp and yp are the coordinates (in radians) of the
///     Celestial Intermediate Pole with respect to the International
///     Terrestrial Reference System (see IERS Conventions 2003),
///     measured along the meridians 0 and 90 deg west respectively.
///
///  4) The matrix rc2t transforms from celestial to terrestrial
///     coordinates:
///
///        [TRS] = RPOM * R_3(ERA) * RC2I * [CRS]
///
///              = rc2t * [CRS]
///
///     where [CRS] is a vector in the Geocentric Celestial Reference
///     System and [TRS] is a vector in the International Terrestrial
///     Reference System (see IERS Conventions 2003), ERA is the Earth
///     Rotation Angle and RPOM is the polar motion matrix.
///
///  5) Although its name does not include "00", This function is in fact
///     specific to the IAU 2000 models.
///
///  Called:
///     iauC2ixy     celestial-to-intermediate matrix, given X,Y
///     iauEra00     Earth rotation angle, IAU 2000
///     iauSp00      the TIO locator s', IERS 2000
///     iauPom00     polar motion matrix
///     iauC2tcio    form CIO-based celestial-to-terrestrial matrix
///
/// Reference:
///
///     McCarthy, D. D., Petit, G. (eds.), IERS Conventions (2003),
///     IERS Technical Note No. 32, BKG (2004)
pub fn c2txy(
    tta: f64,
    ttb: f64,
    uta: f64,
    utb: f64,
    x: f64,
    y: f64,
    xp: f64,
    yp: f64,
) -> [[f64; 3]; 3] {
    /* Form the celestial-to-intermediate matrix for this TT. */
    let rc2i = c2ixy(tta, ttb, x, y);

    /* Predict the Earth rotation angle for this UT1. */
    let era = era00(uta, utb);

    /* Estimate s'. */
    let sp = sp00(tta, ttb);

    /* Form the polar motion matrix. */
    let rpom = pom00(xp, yp, sp);

    /* Combine to form the celestial-to-terrestrial matrix. */
    c2tcio(&rc2i, era, &rpom)
}
