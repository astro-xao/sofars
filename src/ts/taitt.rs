use crate::consts::{DAYSEC, TTMTAI};
pub fn taitt(tai1: f64, tai2: f64) -> Result<(f64, f64), i32> {
    // TT minus TAI (days).
    const DTAT: f64 = TTMTAI / DAYSEC;

    // Result, safeguarding precision.
    let (tt1, tt2) = if tai1.abs() > tai2.abs() {
        (tai1, tai2 + DTAT)
    } else {
        (tai1 + DTAT, tai2)
    };

    // Status (always OK).
    Ok((tt1, tt2))
}