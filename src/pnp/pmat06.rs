use crate::pnp::{fw2m, pfw06};

///  Precession matrix (including frame bias) from GCRS to a specified
///  date, IAU 2006 model.
///
///  Given:
///     date1,date2  f64          TT as a 2-part Julian Date (Note 1)
///
///  Returned (function value):
///                 [[f64; 3]; 3] bias-precession matrix (Note 2)
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
///  2) The matrix operates in the sense V(date) = rbp * V(GCRS), where
///     the p-vector V(GCRS) is with respect to the Geocentric Celestial
///     Reference System (IAU, 2000) and the p-vector V(date) is with
///     respect to the mean equatorial triad of the given date.
///
///  Called:
///     iauPfw06     bias-precession F-W angles, IAU 2006
///     iauFw2m      F-W angles to r-matrix
///
///  References:
///
///     Capitaine, N. & Wallace, P.T., 2006, Astron.Astrophys. 450, 855
///
///     IAU: Trans. International Astronomical Union, Vol. XXIVB;  Proc.
///     24th General Assembly, Manchester, UK.  Resolutions B1.3, B1.6.
///     (2000)
///
///     Wallace, P.T. & Capitaine, N., 2006, Astron.Astrophys. 459, 981
pub fn pmat06(date1: f64, date2: f64) -> [[f64; 3]; 3] {
    /* Bias-precession Fukushima-Williams angles. */
    let (gamb, phib, psib, epsa) = pfw06(date1, date2);

    /* Form the matrix. */
    fw2m(gamb, phib, psib, epsa)
}
