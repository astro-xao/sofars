use crate::consts::{DJM0, IYMIN, MTAB};

/// Gregorian calendar to Julian Day number
pub fn cal2jd(iy: i32, im: i32, id: i32) -> Result<(f64, f64), i32> {
    if iy < IYMIN {
        return Err(-1);
    }

    match im {
        1..=12 => (),
        _ => return Err(-2),
    }

    let ly = if im == 2 && iy % 4 == 0 && (iy % 100 != 0 || iy % 400 == 0) {
        1
    } else {
        0
    };

    if id < 1 || id > (MTAB[(im - 1) as usize] + ly) {
        return Err(-3);
    }

    let my = (im - 14) / 12;
    let iypmy = (iy + my) as i64;
    let djm0 = DJM0;
    let djm = ((1461 * (iypmy + 4800)) / 4 + (367 * ((im - 2 - 12 * my) as i64)) / 12
        - (3 * ((iypmy + 4900) / 100)) / 4
        + (id as i64)
        - 2432076) as f64;
    Ok((djm0, djm))
}

/// Besselian Epoch to Julian Date
pub fn epb2jd() {
    // ...
}

/// Julian Date to Julian Epoch
pub fn epj() {
    // ...
}

/// Julian Epoch to Julian Date
pub fn epj2jd() {
    // ...
}

/// Julian Date to Gregorian date for formatted output
pub fn jdcalf() {
    // ...
}
