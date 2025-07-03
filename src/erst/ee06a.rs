use super::{gmst06, gst06a};
use crate::vm::anpm;

///  Equation of the equinoxes, IAU 2006/2000A
///
///  Equation of the equinoxes, compatible with IAU 2000 resolutions and
///  IAU 2006/2000A precession-nutation.
///
///  This function is part of the International Astronomical Union's
///  SOFA (Standards of Fundamental Astronomy) software collection.
///
///  Status:  support function.
///
///  Given:
///  ```
///     date1,date2  double    TT as a 2-part Julian Date (Note 1)
///  ```
///  Returned (function value):
///  ```
///                  double    equation of the equinoxes (Note 2)
///  ```
///  Notes:
///
///  1) The TT date date1+date2 is a Julian Date, apportioned in any
///     convenient way between the two arguments.  For example,
///     JD(TT)=2450123.7 could be expressed in any of these ways,
///     among others:
///  ```
///            date1          date2
///
///         2450123.7           0.0       (JD method)
///         2451545.0       -1421.3       (J2000 method)
///         2400000.5       50123.2       (MJD method)
///         2450123.5           0.2       (date & time method)
///  ```
///     The JD method is the most natural and convenient to use in
///     cases where the loss of several decimal digits of resolution
///     is acceptable.  The J2000 method is best matched to the way
///     the argument is handled internally and will deliver the
///     optimum resolution.  The MJD method and the date & time methods
///     are both good compromises between resolution and convenience.
///
///  2) The result, which is in radians, operates in the following sense:
///
///        Greenwich apparent ST = GMST + equation of the equinoxes
///
///  Called:
///  ```
///     iauAnpm      normalize angle into range +/- pi
///     iauGst06a    Greenwich apparent sidereal time, IAU 2006/2000A
///     iauGmst06    Greenwich mean sidereal time, IAU 2006
///  ```
///  Reference:
///
///     McCarthy, D. D., Petit, G. (eds.), 2004, IERS Conventions (2003),
///     IERS Technical Note No. 32, BKG
pub fn ee06a(date1: f64, date2: f64) -> f64 {
    let _gst06a = gst06a(0.0, 0.0, date1, date2);
    let _gmst06 = gmst06(0.0, 0.0, date1, date2);

    // Equation of the equinoxes.
    let ee = anpm(_gst06a - _gmst06);

    ee
}
