use super::{bpn2xy, eors, pnm06a, s06};

///  Equation of the origins, IAU 2006 precession and IAU 2000A nutation.
///
///  This function is part of the International Astronomical Union's
///  SOFA (Standards of Fundamental Astronomy) software collection.
///
///  Status:  support function.
///
///  Given:
///     date1,date2  double    TT as a 2-part Julian Date (Note 1)
///
///  Returned (function value):
///                  double    the equation of the origins in radians
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
///  2) The equation of the origins is the distance between the true
///     equinox and the celestial intermediate origin and, equivalently,
///     the difference between Earth rotation angle and Greenwich
///     apparent sidereal time (ERA-GST).  It comprises the precession
///     (since J2000.0) in right ascension plus the equation of the
///     equinoxes (including the small correction terms).
///
///  Called:
///     iauPnm06a    classical NPB matrix, IAU 2006/2000A
///     iauBpn2xy    extract CIP X,Y coordinates from NPB matrix
///     iauS06       the CIO locator s, given X,Y, IAU 2006
///     iauEors      equation of the origins, given NPB matrix and s
///
///  References:
///
///     Capitaine, N. & Wallace, P.T., 2006, Astron.Astrophys. 450, 855
///
///     Wallace, P.T. & Capitaine, N., 2006, Astron.Astrophys. 459, 981
pub fn eo06a(date1: f64, date2: f64) -> f64 {
    /* Classical nutation x precession x bias matrix. */
    let r = pnm06a(date1, date2);

    /* Extract CIP coordinates. */
    let (x, y) = bpn2xy(&r);

    /* The CIO locator, s. */
    let s = s06(date1, date2, x, y);

    /* Solve for the EO. */
    eors(&r, s)
}
