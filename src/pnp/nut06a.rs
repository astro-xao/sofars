use crate::consts::{DJ00, DJC};

pub fn nut06a(date1: f64, date2: f64) -> (f64, f64) {
    /* Interval between fundamental date J2000.0 and given date (JC). */
    let t = ((date1 - DJ00) + date2) / DJC;

    /* Factor correcting for secular variation of J2. */
    let fj2 = -2.7774e-6 * t;

    /* Obtain IAU 2000A nutation. */
    let (dp, de) = super::nut00a(date1, date2);

    /* Apply P03 adjustments (Wallace & Capitaine, 2006, Eqs.5). */
    let dpsi = dp + dp * (0.4697e-6 + fj2);
    let deps = de + de * fj2;

    (dpsi, deps)
}