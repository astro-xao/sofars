use crate::consts::{DAS2R, DJ00, DJC};

pub fn sp00(date1: f64, date2: f64) -> f64 {
    // Interval between fundamental epoch J2000.0 and current date (JC).
    let t = ((date1 - DJ00) + date2) / DJC;

    // Approximate s'.
    -47e-6 * t * DAS2R
}
