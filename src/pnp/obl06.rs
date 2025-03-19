use crate::consts::{DAS2R, DJ00, DJC};

pub fn obl06(date1: f64, date2: f64) -> f64 {
    let (t, eps0): (f64, f64);

    /* Interval between fundamental date J2000.0 and given date (JC). */
    t = ((date1 - DJ00) + date2) / DJC;

    /* Mean obliquity */
    eps0 = (84381.406
        + (-46.836769
            + (-0.0001831 + (0.00200340 + (-0.000000576 + (-0.0000000434) * t) * t) * t) * t)
            * t)
        * DAS2R;

    eps0
}
