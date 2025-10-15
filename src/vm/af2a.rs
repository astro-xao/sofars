use crate::consts::DAS2R;

/// Convert degrees, arcminutes, arcseconds to radians.
pub fn af2a(s: char, ideg: i32, iamin: i32, asec: f64) -> Result<f64, i32> {
    let rad: f64;
    rad = match s {
        '-' => -1.0,
        _ => 1.0,
    } * (60.0 * (60.0 * ideg as f64 + iamin as f64) + asec as f64)
        * DAS2R;

    /* Validate arguments and return status. */
    if ideg < 0 || ideg > 359 {
        return Err(1);
    }
    if iamin < 0 || iamin > 59 {
        return Err(2);
    }
    if asec < 0.0 || asec >= 60.0 {
        return Err(3);
    }

    Ok(rad)
}
