use super::pn00b::pn00b;

///  Form the matrix of precession-nutation for a given date (including
///  frame bias), equinox-based, IAU 2000B model.
///
///  Given:
///     date1,date2 f64          TT as a 2-part Julian Date (Note 1)
///
///  Returned (function value):
///                 [[f64; 3]; 3] bias-precession-nutation matrix (Note 2)
///
///  Notes:
///
///  1) The TT date date1+date2 is a Julian Date, apportioned in any
///     convenient way between the two arguments.  For example,
///     JD(TT)=2450123.7 could be expressed in any of these ways,
///     among others:
///
///            date1          date2
///
///         2450123.7           0.0       (JD method)
///         2451545.0       -1421.3       (J2000 method)
///         2400000.5       50123.2       (MJD method)
///         2450123.5           0.2       (date & time method)
///
///     The JD method is the most natural and convenient to use in
///     cases where the loss of several decimal digits of resolution
///     is acceptable.  The J2000 method is best matched to the way
///     the argument is handled internally and will deliver the
///     optimum resolution.  The MJD method and the date & time methods
///     are both good compromises between resolution and convenience.
///
///  2) The matrix operates in the sense V(date) = rbpn * V(GCRS), where
///     the p-vector V(date) is with respect to the true equatorial triad
///     of date date1+date2 and the p-vector V(GCRS) is with respect to
///     the Geocentric Celestial Reference System (IAU, 2000).
///
///  3) A faster, but slightly less accurate, result (about 1 mas), than
///     the iauPnm00a function.
///
///  Called:
///     iauPn00b     bias/precession/nutation, IAU 2000B
///
///  Reference:
///
///     Capitaine, N., Chapront, J., Lambert, S. and Wallace, P.,
///     "Expressions for the Celestial Intermediate Pole and Celestial
///     Ephemeris Origin consistent with the IAU 2000A precession-
///     nutation model", Astron.Astrophys. 400, 1145-1154 (2003)
pub fn pnm00b(date1: f64, date2: f64) -> [[f64; 3]; 3] {
    /* Obtain the required matrix (discarding other results). */
    let (_, _, _, _, _, _, _, rbpn) = pn00b(date1, date2);
    rbpn
}
