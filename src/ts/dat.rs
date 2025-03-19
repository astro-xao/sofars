///  For a given UTC date, calculate Delta(AT) = TAI-UTC.
///  ```
///     :------------------------------------------:
///     :                                          :
///     :                 IMPORTANT                :
///     :                                          :
///     :  A new version of this function must be  :
///     :  produced whenever a new leap second is  :
///     :  announced.  There are four items to     :
///     :  change on each such occasion:           :
///     :                                          :
///     :  1) A new line must be added to the set  :
///     :     of statements that initialize the    :
///     :     array "changes".                     :
///     :                                          :
///     :  2) The constant IYV must be set to the  :
///     :     current year.                        :
///     :                                          :
///     :  3) The "Latest leap second" comment     :
///     :     below must be set to the new leap    :
///     :     second date.                         :
///     :                                          :
///     :  4) The "This revision" comment, later,  :
///     :     must be set to the current date.     :
///     :                                          :
///     :  Change (2) must also be carried out     :
///     :  whenever the function is re-issued,     :
///     :  even if no leap seconds have been       :
///     :  added.                                  :
///     :                                          :
///     :  Latest leap second:  2016 December 31   :
///     :                                          :
///     :__________________________________________:
///  ```
///  This function is part of the International Astronomical Union's
///  SOFA (Standards of Fundamental Astronomy) software collection.
///
///  Status:  user-replaceable support function.
///
///  Given:
///  ```
///     iy     int      UTC:  year (Notes 1 and 2)
///     im     int            month (Note 2)
///     id     int            day (Notes 2 and 3)
///     fd     double         fraction of day (Note 4)
///  ```
///  Returned:
///  ```
///     deltat double   TAI minus UTC, seconds
///  ```
///  Returned (function value):
///  ```
///            int      status (Note 5):
///                       1 = dubious year (Note 1)
///                       0 = OK
///                      -1 = bad year
///                      -2 = bad month
///                      -3 = bad day (Note 3)
///                      -4 = bad fraction (Note 4)
///                      -5 = internal error (Note 5)
///  ```
///  Notes:
///
///  1) UTC began at 1960 January 1.0 (JD 2436934.5) and it is improper
///     to call the function with an earlier date.  If this is attempted,
///     zero is returned together with a warning status.
///
///     Because leap seconds cannot, in principle, be predicted in
///     advance, a reliable check for dates beyond the valid range is
///     impossible.  To guard against gross errors, a year five or more
///     after the release year of the present function (see the constant
///     IYV) is considered dubious.  In this case a warning status is
///     returned but the result is computed in the normal way.
///
///     For both too-early and too-late years, the warning status is +1.
///     This is distinct from the error status -1, which signifies a year
///     so early that JD could not be computed.
///
///  2) If the specified date is for a day which ends with a leap second,
///     the TAI-UTC value returned is for the period leading up to the
///     leap second.  If the date is for a day which begins as a leap
///     second ends, the TAI-UTC returned is for the period following the
///     leap second.
///
///  3) The day number must be in the normal calendar range, for example
///     1 through 30 for April.  The "almanac" convention of allowing
///     such dates as January 0 and December 32 is not supported in this
///     function, in order to avoid confusion near leap seconds.
///
///  4) The fraction of day is used only for dates before the
///     introduction of leap seconds, the first of which occurred at the
///     end of 1971.  It is tested for validity (0 to 1 is the valid
///     range) even if not used;  if invalid, zero is used and status -4
///     is returned.  For many applications, setting fd to zero is
///     acceptable;  the resulting error is always less than 3 ms (and
///     occurs only pre-1972).
///
///  5) The status value returned in the case where there are multiple
///     errors refers to the first error detected.  For example, if the
///     month and day are 13 and 32 respectively, status -2 (bad month)
///     will be returned.  The "internal error" status refers to a
///     case that is impossible but causes some compilers to issue a
///     warning.
///
///  6) In cases where a valid result is not available, zero is returned.
///
///  References:
///
///  1) For dates from 1961 January 1 onwards, the expressions from the
///     file ftp://maia.usno.navy.mil/ser7/tai-utc.dat are used.
///
///  2) The 5ms timestep at 1961 January 1 is taken from 2.58.1 (p87) of
///     the 1992 Explanatory Supplement.
///
///  Called:
///     iauCal2jd    Gregorian calendar to JD
pub fn dat(iy: i32, im: i32, id: i32, fd: f64) -> Result<f64, i32> {
    // Release year for this version of iauDat
    const IYV: i32 = 2023;

    // Reference dates (MJD) and drift rates (s/day), pre leap seconds
    const DRIFT: &[(f64, f64)] = &[
        (37300.0, 0.0012960),
        (37300.0, 0.0012960),
        (37300.0, 0.0012960),
        (37665.0, 0.0011232),
        (37665.0, 0.0011232),
        (38761.0, 0.0012960),
        (38761.0, 0.0012960),
        (38761.0, 0.0012960),
        (38761.0, 0.0012960),
        (38761.0, 0.0012960),
        (38761.0, 0.0012960),
        (38761.0, 0.0012960),
        (39126.0, 0.0025920),
        (39126.0, 0.0025920),
    ];

    // Number of Delta(AT) expressions before leap seconds were introduced
    const NERA1: usize = DRIFT.len();

    // Dates and Delta(AT)s
    const CHANGES: &[(i32, i32, f64)] = &[
        (1960, 1, 1.4178180),
        (1961, 1, 1.4228180),
        (1961, 8, 1.3728180),
        (1962, 1, 1.8458580),
        (1963, 11, 1.9458580),
        (1964, 1, 3.2401300),
        (1964, 4, 3.3401300),
        (1964, 9, 3.4401300),
        (1965, 1, 3.5401300),
        (1965, 3, 3.6401300),
        (1965, 7, 3.7401300),
        (1965, 9, 3.8401300),
        (1966, 1, 4.3131700),
        (1968, 2, 4.2131700),
        (1972, 1, 10.0),
        (1972, 7, 11.0),
        (1973, 1, 12.0),
        (1974, 1, 13.0),
        (1975, 1, 14.0),
        (1976, 1, 15.0),
        (1977, 1, 16.0),
        (1978, 1, 17.0),
        (1979, 1, 18.0),
        (1980, 1, 19.0),
        (1981, 7, 20.0),
        (1982, 7, 21.0),
        (1983, 7, 22.0),
        (1985, 7, 23.0),
        (1988, 1, 24.0),
        (1990, 1, 25.0),
        (1991, 1, 26.0),
        (1992, 7, 27.0),
        (1993, 7, 28.0),
        (1994, 7, 29.0),
        (1996, 1, 30.0),
        (1997, 7, 31.0),
        (1999, 1, 32.0),
        (2006, 1, 33.0),
        (2009, 1, 34.0),
        (2012, 7, 35.0),
        (2015, 7, 36.0),
        (2017, 1, 37.0),
    ];

    // Number of Delta(AT) changes
    const NDAT: usize = CHANGES.len();

    // Miscellaneous local variables
    let mut i: usize;
    let m: i32;
    let mut da: f64;

    // Initialize the result to zero.
    let mut deltat: f64 = 0.0;

    // If invalid fraction of a day, set error status and give up.
    if fd < 0.0 || fd > 1.0 {
        return Err(-4);
    }

    use crate::cal::cal2jd;
    // Convert the date into an MJD.
    let j = cal2jd(iy, im, id);

    // If invalid year, month, or day, give up.
    if j.is_err() {
        return Err(j.err().unwrap());
    }

    let (_, djm) = j.unwrap();

    // If pre-UTC year, set warning status and give up.
    if iy < CHANGES[0].0 {
        return Err(1);
    }

    let j: i32;
    // If suspiciously late year, set warning status but proceed.
    if iy > IYV + 5 {
        j = 1;
    }

    // Combine year and month to form a date-ordered integer...
    m = 12 * iy + im;

    // ...and use it to find the preceding table entry.
    i = NDAT - 1;
    while i > 0 && m < (12 * CHANGES[i].0 + CHANGES[i].1) {
        i -= 1;
    }

    // Prevent underflow warnings.
    if i == 0 && m < (12 * CHANGES[i].0 + CHANGES[i].1) {
        return Err(-5);
    }

    // Get the Delta(AT).
    da = CHANGES[i].2;

    // If pre-1972, adjust for drift.
    if i < NERA1 {
        da += (djm + fd - DRIFT[i].0) * DRIFT[i].1;
    }

    // Return the Delta(AT) value.
    deltat = da;

    // Return the status.
    Ok(deltat)
}
