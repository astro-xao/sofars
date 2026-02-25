use crate::consts::DAYSEC;

///  Time scale transformation:  Terrestrial Time, TT, to Barycentric
///  Dynamical Time, TDB.
///
///  This function is part of the International Astronomical Union's
///  SOFA (Standards of Fundamental Astronomy) software collection.
///
///  Status:  canonical.
///
///  Given:
///     tt1,tt2    double    TT as a 2-part Julian Date
///     dtr        double    TDB-TT in seconds
///
///  Returned:
///     tdb1,tdb2  double    TDB as a 2-part Julian Date
///
///  Returned (function value):
///                int       status:  0 = OK
///
pub fn tttdb(tt1: f64, tt2: f64, dtr: f64) -> Result<(f64, f64), i32> {
    let dtrd = dtr / DAYSEC;
    let (tdb1, tdb2);

    /* Result, safeguarding precision. */
    if tt1.abs() > tt2.abs() {
        tdb1 = tt1;
        tdb2 = tt2 + dtrd;
    } else {
        tdb1 = tt1 + dtrd;
        tdb2 = tt2;
    }

    Ok((tdb1, tdb2))
}
