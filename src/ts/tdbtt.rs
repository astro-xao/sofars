use crate::consts::DAYSEC;

///  Time scale transformation:  Barycentric Dynamical Time, TDB, to
///  Terrestrial Time, TT.
///
///  This function is part of the International Astronomical Union's
///  SOFA (Standards of Fundamental Astronomy) software collection.
///
///  Status:  canonical.
///
///  Given:
///     tdb1,tdb2  double    TDB as a 2-part Julian Date
///     dtr        double    TDB-TT in seconds
///
///  Returned:
///     tt1,tt2    double    TT as a 2-part Julian Date
///
///  Returned (function value):
///                int       status:  0 = OK
///
pub fn tdbtt(tdb1: f64, tdb2: f64, dtr: f64) -> Result<(f64, f64), i32> {
    let dtrd = dtr / DAYSEC;
    let (tt1, tt2);

    /* Result, safeguarding precision. */
    if tdb1.abs() > tdb2.abs() {
        tt1 = tdb1;
        tt2 = tdb2 - dtrd;
    } else {
        tt1 = tdb1 - dtrd;
        tt2 = tdb2;
    }

    Ok((tt1, tt2))
}
