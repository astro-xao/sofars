use crate::consts::DAYSEC;

///  Time scale transformation:  Universal Time, UT1, to International
///  Atomic Time, TAI.
///
///  This function is part of the International Astronomical Union's
///  SOFA (Standards of Fundamental Astronomy) software collection.
///
///  Status:  canonical.
///
///  Given:
///     ut11,ut12  double    UT1 as a 2-part Julian Date
///     dta        double    UT1-TAI in seconds
///
///  Returned:
///     tai1,tai2  double    TAI as a 2-part Julian Date
///
///  Returned (function value):
///                int       status:  0 = OK
///
pub fn ut1tai(ut11: f64, ut12: f64, dta: f64) -> Result<(f64, f64), i32> {
    let dtad = dta / DAYSEC;
    let (tai1, tai2);

    /* Result, safeguarding precision. */
    if ut11.abs() > ut12.abs() {
        tai1 = ut11;
        tai2 = ut12 - dtad;
    } else {
        tai1 = ut11 - dtad;
        tai2 = ut12;
    }

    Ok((tai1, tai2))
}
