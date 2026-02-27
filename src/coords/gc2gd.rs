use crate::coords::{eform, gc2gde};

/// Transform geocentric coordinates to geodetic using the specified
/// reference ellipsoid.
///
/// Status:  canonical transformation.
///
/// Given:
///    n       i32        ellipsoid identifier (Note 1)
///    xyz     [f64; 3]  geocentric vector (Note 2)
///
/// Returned:
///    (elong, phi, height)   (f64, f64, f64)
///
/// Returned (function value):
///    Result<(f64, f64, f64), i32> status:  0 = OK
///                                         -1 = illegal identifier (Note 3)
///                                         -2 = internal error (Note 3)
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
/// 2) The geocentric vector (xyz, given) and height (height, returned)
///    are in meters.
///
/// 3) An error status -1 means that the identifier n is illegal.  An
///    error status -2 is theoretically impossible.  In all error cases,
///    the error status is returned.
pub fn gc2gd(n: i32, xyz: [f64; 3]) -> Result<(f64, f64, f64), i32> {
    /* Obtain reference ellipsoid parameters. */
    let (a, f) = match eform(n) {
        Ok(res) => res,
        Err(e) => return Err(e),
    };

    /* If OK, transform x,y,z to longitude, geodetic latitude, height. */
    match gc2gde(a, f, xyz) {
        Ok(res) => Ok(res),
        Err(_) => Err(-2),
    }
}
