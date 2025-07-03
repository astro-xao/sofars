use crate::consts::DAYSEC;

///  Time scale transformation:  Universal Time, UT1, to Terrestrial
///  Time, TT.
///
///  This function is part of the International Astronomical Union's
///  SOFA (Standards of Fundamental Astronomy) software collection.
///
///  Status:  canonical.
///
///  Given:
///  ```
///     ut11,ut12  double    UT1 as a 2-part Julian Date
///     dt         double    TT-UT1 in seconds
///  ```
///  Returned:
///  ```
///     tt1,tt2    double    TT as a 2-part Julian Date
///  ```
///  Returned (function value):
///  ```
///                int       status:  0 = OK
///  ```
///  Notes:
///
///  1) ut11+ut12 is Julian Date, apportioned in any convenient way
///     between the two arguments, for example where ut11 is the Julian
///     Day Number and ut12 is the fraction of a day.  The returned
///     tt1,tt2 follow suit.
///
///  2) The argument dt is classical Delta T.
///
///  Reference:
///
///     Explanatory Supplement to the Astronomical Almanac,
///     P. Kenneth Seidelmann (ed), University Science Books (1992)
///
pub fn ut1tt(ut11: f64, ut12: f64, dt: f64) -> Result<(f64, f64), i32> {
    let dtd = dt / DAYSEC;

    // Result, safeguarding precision.
    let (tt1, tt2) = if ut11.abs() > ut12.abs() {
        (ut11, ut12 + dtd)
    } else {
        (ut11 + dtd, ut12)
    };

    /* Status (always OK). */
    Ok((tt1, tt2))
}
