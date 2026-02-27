use crate::pnp::{obl06, pmat06};
use crate::vm::{ir, rx, rxr};

/// ICRS equatorial to ecliptic rotation matrix, IAU 2006.
///
/// Status:  support function.
///
/// Given:
///    date1,date2  f64         TT as a 2-part Julian date (Note 1)
///
/// Returned:
///    rm           [[f64; 3]; 3]   ICRS to ecliptic rotation matrix
///
/// Notes:
///
/// 1) The TT date date1+date2 is a Julian Date, apportioned in any
///    convenient way between the two arguments.  For example,
///    JD(TT)=2450123.7 could be expressed in any of these ways,
///    among others:
///
///           date1          date2
///
///        2450123.7           0.0       (JD method)
///        2451545.0       -1421.3       (J2000 method)
///        2400000.5       50123.2       (MJD method)
///        2450123.5           0.2       (date & time method)
///
///    The JD method is the most natural and convenient to use in
///    cases where the loss of several decimal digits of resolution
///    is acceptable.  The J2000 method is best matched to the way
///    the argument is handled internally and will deliver the
///    optimum resolution.  The MJD method and the date & time methods
///    are both good compromises between resolution and convenience.
///
/// 2) The matrix is in the sense
///
///       E_ep = rm x P_ICRS,
///
///    where P_ICRS is a vector with respect to ICRS right ascension
///    and declination axes and E_ep is the same vector with respect to
///    the (inertial) ecliptic and equinox of date.
///
///    P_ICRS is a free vector, merely a direction, typically of unit
///    magnitude, and not bound to any particular spatial origin, such
///    as the Earth, Sun or SSB.  No assumptions are made about whether
///    it represents starlight and embodies astrometric effects such as
///    parallax or aberration.  The transformation is approximately that
///    between mean J2000.0 right ascension and declination and ecliptic
///    longitude and latitude, with only frame bias (always less than
///    25 mas) to disturb this classical picture.
pub fn ecm06(date1: f64, date2: f64) -> [[f64; 3]; 3] {
    let mut rm = [[0.0; 3]; 3];
    let mut e = [[0.0; 3]; 3];

    /* Obliquity, IAU 2006. */
    let ob = obl06(date1, date2);

    /* Precession-bias matrix, IAU 2006. */
    let bp = pmat06(date1, date2);

    /* Equatorial of date to ecliptic matrix. */
    ir(&mut e);
    rx(ob, &mut e);

    /* ICRS to ecliptic coordinates rotation matrix, IAU 2006. */
    rxr(&e, &bp, &mut rm);

    rm
}
