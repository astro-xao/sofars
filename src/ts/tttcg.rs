use crate::consts::{DAYSEC, DJM0, DJM77, ELG, TTMTAI};

///  Time scale transformation:  Terrestrial Time, TT, to Geocentric
///  Coordinate Time, TCG.
///
///  This function is part of the International Astronomical Union's
///  SOFA (Standards of Fundamental Astronomy) software collection.
///
///  Status:  canonical.
///
///  Given:
///     tt1,tt2    double    TT as a 2-part Julian Date
///
///  Returned:
///     tcg1,tcg2  double    TCG as a 2-part Julian Date
///
///  Returned (function value):
///                int       status:  0 = OK
///
pub fn tttcg(tt1: f64, tt2: f64) -> Result<(f64, f64), i32> {
    /* 1977 Jan 1 00:00:32.184 TT, as MJD */
    let t77t: f64 = DJM77 + TTMTAI / DAYSEC;

    /* TT to TCG rate */
    let elgg: f64 = ELG / (1.0 - ELG);

    let (tcg1, tcg2);

    /* Result, safeguarding precision. */
    if tt1.abs() > tt2.abs() {
        tcg1 = tt1;
        tcg2 = tt2 + ((tt1 - DJM0) + (tt2 - t77t)) * elgg;
    } else {
        tcg1 = tt1 + ((tt2 - DJM0) + (tt1 - t77t)) * elgg;
        tcg2 = tt2;
    }

    Ok((tcg1, tcg2))
}
