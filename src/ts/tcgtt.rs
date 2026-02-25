use crate::consts::{DAYSEC, DJM0, DJM77, ELG, TTMTAI};

///  Time scale transformation:  Geocentric Coordinate Time, TCG, to
///  Terrestrial Time, TT.
///
///  This function is part of the International Astronomical Union's
///  SOFA (Standards of Fundamental Astronomy) software collection.
///
///  Status:  canonical.
///
///  Given:
///     tcg1,tcg2  double    TCG as a 2-part Julian Date
///
///  Returned:
///     tt1,tt2    double    TT as a 2-part Julian Date
///
///  Returned (function value):
///                int       status:  0 = OK
///
pub fn tcgtt(tcg1: f64, tcg2: f64) -> Result<(f64, f64), i32> {
    /* 1977 Jan 1 00:00:32.184 TT, as MJD */
    let t77t: f64 = DJM77 + TTMTAI / DAYSEC;

    let (tt1, tt2);

    /* Result, safeguarding precision. */
    if tcg1.abs() > tcg2.abs() {
        tt1 = tcg1;
        tt2 = tcg2 - ((tcg1 - DJM0) + (tcg2 - t77t)) * ELG;
    } else {
        tt1 = tcg1 - ((tcg2 - DJM0) + (tcg1 - t77t)) * ELG;
        tt2 = tcg2;
    }

    Ok((tt1, tt2))
}
