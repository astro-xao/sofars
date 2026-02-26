#![allow(non_snake_case)]
use crate::consts::{D2PI, DAYSEC};
use crate::coords::gd2gc;
use crate::pnp::pom00;
use crate::vm::trxp;

///  Observatory position and velocity
///
///  Position and velocity of a terrestrial observing station.
///
///  This function is part of the International Astronomical Union's
///  SOFA (Standards of Fundamental Astronomy) software collection.
///
///  Status:  support function.
///
///  Given:
///  ```
///     elong   double       longitude (radians, east +ve, Note 1)
///     phi     double       latitude (geodetic, radians, Note 1)
///     hm      double       height above ref. ellipsoid (geodetic, m)
///     xp,yp   double       coordinates of the pole (radians, Note 2)
///     sp      double       the TIO locator s' (radians, Note 2)
///     theta   double       Earth rotation angle (radians, Note 3)
///  ```
///  Returned:
///  ```
///     pv      double[2][3] position/velocity vector (m, m/s, CIRS)
///  ```
///  Notes:
///
///  1) The terrestrial coordinates are with respect to the WGS84
///     reference ellipsoid.
///
///  2) xp and yp are the coordinates (in radians) of the Celestial
///     Intermediate Pole with respect to the International Terrestrial
///     Reference System (see IERS Conventions), measured along the
///     meridians 0 and 90 deg west respectively.  sp is the TIO locator
///     s', in radians, which positions the Terrestrial Intermediate
///     Origin on the equator.  For many applications, xp, yp and
///     (especially) sp can be set to zero.
///
///  3) If theta is Greenwich apparent sidereal time instead of Earth
///     rotation angle, the result is with respect to the true equator
///     and equinox of date, i.e. with the x-axis at the equinox rather
///     than the celestial intermediate origin.
///
///  4) The velocity units are meters per UT1 second, not per SI second.
///     This is unlikely to have any practical consequences in the modern
///     era.
///
///  5) No validation is performed on the arguments.  Error cases that
///     could lead to arithmetic exceptions are trapped by the iauGd2gc
///     function, and the result set to zeros.
///
///  References:
///
///     McCarthy, D. D., Petit, G. (eds.), IERS Conventions (2003),
///     IERS Technical Note No. 32, BKG (2004)
///
///     Urban, S. & Seidelmann, P. K. (eds), Explanatory Supplement to
///     the Astronomical Almanac, 3rd ed., University Science Books
///     (2013), Section 7.4.3.3.
///
///  Called:
///  ```
///     iauGd2gc     geodetic to geocentric transformation
///     iauPom00     polar motion matrix
///     iauTrxp      product of transpose of r-matrix and p-vector
///  ```
pub fn pvtob(
    elong: f64,
    phi: f64,
    hm: f64,
    xp: f64,
    yp: f64,
    sp: f64,
    theta: f64,
    pv: &mut [[f64; 3]; 2],
) {
    /* Earth rotation rate in radians per UT1 second */
    let OM = 1.00273781191135448 * D2PI / DAYSEC;

    let xyzm = &mut [0.0; 3];
    let xyz = &mut [0.0; 3];
    let (x, y, z, s, c): (f64, f64, f64, f64, f64);

    /* Geodetic to geocentric transformation (WGS84). */
    #[allow(unused_must_use)]
    gd2gc(1, elong, phi, hm, xyzm);

    /* Polar motion and TIO position. */
    let rpm = pom00(xp, yp, sp);
    trxp(&rpm, xyzm, xyz);
    x = xyz[0];
    y = xyz[1];
    z = xyz[2];

    /* Functions of ERA. */
    s = theta.sin();
    c = theta.cos();

    /* Position. */
    pv[0][0] = c * x - s * y;
    pv[0][1] = s * x + c * y;
    pv[0][2] = z;

    /* Velocity. */
    pv[1][0] = OM * (-s * x - c * y);
    pv[1][1] = OM * (c * x - s * y);
    pv[1][2] = 0.0;
}
