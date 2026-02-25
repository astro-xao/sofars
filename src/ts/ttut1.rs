use crate::consts::DAYSEC;

///  Time scale transformation:  Terrestrial Time, TT, to Universal Time,
///  UT1.
///
///  This function is part of the International Astronomical Union's
///  SOFA (Standards of Fundamental Astronomy) software collection.
///
///  Status:  canonical.
///
///  Given:
///     tt1,tt2    double    TT as a 2-part Julian Date
///     dt         double    TT-UT1 in seconds
///
///  Returned:
///     ut11,ut12  double    UT1 as a 2-part Julian Date
///
///  Returned (function value):
///                int       status:  0 = OK
///
pub fn ttut1(tt1: f64, tt2: f64, dt: f64) -> Result<(f64, f64), i32> {
    let dtd = dt / DAYSEC;
    let (ut11, ut12);

    /* Result, safeguarding precision. */
    if tt1.abs() > tt2.abs() {
        ut11 = tt1;
        ut12 = tt2 - dtd;
    } else {
        ut11 = tt1 - dtd;
        ut12 = tt2;
    }

    Ok((ut11, ut12))
}
