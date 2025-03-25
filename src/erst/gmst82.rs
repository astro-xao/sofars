use crate::consts::{DAYSEC, DJ00, DJC, DS2R};
use crate::vm::anp;
use std::ops::Rem;

/// Greenwich mean sidereal time, IAU 1982
pub fn gmst82(dj1: f64, dj2: f64) -> f64 {
    // Coefficients of IAU 1982 GMST-UT1 model
    let a = 24110.54841 - DAYSEC / 2.0;
    let b = 8640184.812866;
    let c = 0.093104;
    let d = -6.2e-6;

    // The first constant, A, has to be adjusted by 12 hours because the
    // UT1 is supplied as a Julian date, which begins at noon.

    let (d1, d2) = if dj1 < dj2 { (dj1, dj2) } else { (dj2, dj1) };

    // Julian centuries since fundamental epoch.
    let t = (d1 + (d2 - DJ00)) / DJC;

    // Fractional part of JD(UT1), in seconds.
    let f = DAYSEC * (d1.rem(1.0) + d2.rem(1.0));

    // GMST at this UT1.
    let gmst = anp(DS2R * ((a + (b + (c + d * t) * t) * t) + f));

    gmst
}
