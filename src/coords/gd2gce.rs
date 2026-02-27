/// Transform geodetic coordinates to geocentric for a reference
/// ellipsoid of specified form.
///
/// Status:  support function.
///
/// Given:
///    a       f64     equatorial radius (Notes 1,3,4)
///    f       f64     flattening (Notes 2,4)
///    elong   f64     longitude (radians, east +ve, Note 4)
///    phi     f64     latitude (geodetic, radians, Note 4)
///    height  f64     height above ellipsoid (geodetic, Notes 3,4)
///
/// Returned:
///    xyz     [f64; 3]  geocentric vector (Note 3)
///
/// Returned (function value):
///    Result<[f64; 3], i32>   Ok(xyz) or Err(-1) = illegal case (Note 4)
///
/// Notes:
///
/// 1) The equatorial radius, a, can be in any units, but meters is
///    the conventional choice.
///
/// 2) The flattening, f, is (for the Earth) a value around 0.00335,
///    i.e. around 1/298.
///
/// 3) The equatorial radius, a, and the height, height, must be
///    given in the same units, and determine the units of the
///    returned geocentric vector, xyz.
///
/// 4) No validation is performed on individual arguments.  The error
///    status -1 protects against (unrealistic) cases that would lead
///    to arithmetic exceptions.
pub fn gd2gce(a: f64, f: f64, elong: f64, phi: f64, height: f64) -> Result<[f64; 3], i32> {
    /* Functions of geodetic latitude. */
    let (sp, cp) = phi.sin_cos();
    let w = (1.0 - f) * (1.0 - f);
    let d = cp * cp + w * sp * sp;
    if d <= 0.0 {
        return Err(-1);
    }
    let ac = a / d.sqrt();
    let as_ = w * ac;

    /* Geocentric vector. */
    let r = (ac + height) * cp;
    let x = r * elong.cos();
    let y = r * elong.sin();
    let z = (as_ + height) * sp;

    Ok([x, y, z])
}
