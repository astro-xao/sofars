use crate::consts::{DAYSEC, TTMTAI};

///  Time scale transformation:  Terrestrial Time, TT, to International
///  Atomic Time, TAI.
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
///     tai1,tai2  double    TAI as a 2-part Julian Date
///
///  Returned (function value):
///                int       status:  0 = OK
///
pub fn tttai(tt1: f64, tt2: f64) -> Result<(f64, f64), i32> {
    /* TT minus TAI (days). */
    let dtat = TTMTAI / DAYSEC;

    let (tai1, tai2);

    /* Result, safeguarding precision. */
    if tt1.abs() > tt2.abs() {
        tai1 = tt1;
        tai2 = tt2 - dtat;
    } else {
        tai1 = tt1 - dtat;
        tai2 = tt2;
    }

    Ok((tai1, tai2))
}
