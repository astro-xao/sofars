use crate::consts::DS2R;

pub fn tf2a(s: char, ihour: i32, imin: i32, sec: f64) -> Result<f64, i32> {
    /* Compute the interval. */
    let rad = (match s {
        '-' => -1.0,
        _ => 1.0,
    }) * (60.0 * (60.0 * ihour.abs() as f64 + imin.abs() as f64) + sec.abs())
        * DS2R;

    /* Validate arguments and return status. */
    if ihour < 0 || ihour > 23 {
        return Err(1);
    }
    if imin < 0 || imin > 59 {
        return Err(2);
    }
    if sec < 0.0 || sec >= 60.0 {
        return Err(3);
    }

    Ok(rad)
}
