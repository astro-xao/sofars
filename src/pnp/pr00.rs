use crate::consts::{DAS2R, DJ00, DJC};

pub fn pr00(date1: f64, date2: f64) -> (f64, f64) {
    /* Precession and obliquity corrections (radians per century) */
    const PRECOR: f64 = -0.29965 * DAS2R;
    const OBLCOR: f64 = -0.02524 * DAS2R;


    /* Interval between fundamental epoch J2000.0 and given date (JC). */
    let t = ((date1 - DJ00) + date2) / DJC;

    /* Precession rate contributions with respect to IAU 1976/80. */
    let dpsipr = PRECOR * t;
    let depspr = OBLCOR * t;

    (dpsipr, depspr)
}