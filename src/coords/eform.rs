use crate::consts::{GRS80, WGS72, WGS84};

/// Earth reference ellipsoids.
///
/// Status:  canonical.
///
/// Given:
///    n    i32         ellipsoid identifier (Note 1)
///
/// Returned:
///    (a, f)    (f64, f64)      equatorial radius (meters) and flattening (Note 2)
///
/// Returned (function value):
///    Result<(f64, f64), i32>   Ok((a, f)) or Err(-1) = illegal identifier (Note 3)
///
/// Notes:
///
/// 1) The identifier n is a number that specifies the choice of
///    reference ellipsoid.  The following are supported:
///
///       n    ellipsoid
///
///       1     WGS84
///       2     GRS80
///       3     WGS72
///
///    The n value has no significance outside the SOFA software.
///
/// 2) The ellipsoid parameters are returned in the form of equatorial
///    radius in meters (a) and flattening (f).  The latter is a number
///    around 0.00335, i.e. around 1/298.
///
/// 3) For the case where an unsupported n value is supplied, zero a and
///    f are returned, as well as error status.
pub fn eform(n: i32) -> Result<(f64, f64), i32> {
    /* Look up a and f for the specified reference ellipsoid. */
    match n {
        WGS84 => Ok((6378137.0, 1.0 / 298.257223563)),
        GRS80 => Ok((6378137.0, 1.0 / 298.257222101)),
        WGS72 => Ok((6378135.0, 1.0 / 298.26)),
        _ => Err(-1),
    }
}
