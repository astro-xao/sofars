use crate::coords::{eform, gd2gce};

/// Transform geodetic coordinates to geocentric using the specified
/// reference ellipsoid.
///
/// Status:  canonical transformation.
///
/// Given:
///    n       i32        ellipsoid identifier (Note 1)
///    elong   f64        longitude (radians, east +ve, Note 3)
///    phi     f64        latitude (geodetic, radians, Note 3)
///    height  f64        height above ellipsoid (geodetic, Notes 2,3)
///
/// Returned:
///    xyz     [f64; 3]   geocentric vector (Note 2)
///
/// Returned (function value):
///    Result<[f64; 3], i32> status:  0 = OK
///                                  -1 = illegal identifier (Note 3)
///                                  -2 = illegal case (Note 3)
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
/// 2) The height (height, given) and the geocentric vector (xyz,
///    returned) are in meters.
///
/// 3) No validation is performed on the arguments elong, phi and
///    height.  An error status -1 means that the identifier n is
///    illegal.  An error status -2 protects against cases that would
///    lead to arithmetic exceptions.
pub fn gd2gc(n: i32, elong: f64, phi: f64, height: f64) -> Result<[f64; 3], i32> {
    /* Obtain reference ellipsoid parameters. */
    let (a, f) = match eform(n) {
        Ok(res) => res,
        Err(e) => return Err(e),
    };

    /* If OK, transform longitude, geodetic latitude, height to x,y,z. */
    match gd2gce(a, f, elong, phi, height) {
        Ok(res) => Ok(res),
        Err(_) => Err(-2),
    }
}
