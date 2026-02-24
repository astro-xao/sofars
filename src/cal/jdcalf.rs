use crate::cal::jd2cal;

/// Julian Date to Gregorian Calendar, expressed in a form convenient
/// for formatting messages: rounded to a specified precision.
///
/// This function is part of the International Astronomical Union's
/// SOFA (Standards of Fundamental Astronomy) software collection.
///
/// Status: support function.
///
/// # Given:
/// * `ndp`: number of decimal places of days in fraction
/// * `dj1`, `dj2`: dj1+dj2 = Julian Date (Note 1)
///
/// # Returned:
/// * `(iy, im, id, iymdf_3)`: year, month, day, fraction in Gregorian calendar
///
/// # Returned (function value):
/// * `Result`:
///     * `Ok(status)`:
///         * `0` = OK
///         * `1` = ndp not 0-9 (interpreted as 0)
///     * `Err(status)`:
///         * `-1` = date out of range
///
/// # Notes:
/// 1) The Julian Date is apportioned in any convenient way between
///    the arguments dj1 and dj2. For example, JD=2450123.7 could
///    be expressed in any of these ways, among others:
///
///            dj1            dj2
///
///        2450123.7           0.0       (JD method)
///        2451545.0       -1421.3       (J2000 method)
///        2400000.5       50123.2       (MJD method)
///        2450123.5           0.2       (date & time method)
///
/// 2) In early eras the conversion is from the "Proleptic Gregorian
///    Calendar"; no account is taken of the date(s) of adoption of
///    the Gregorian Calendar, nor is the AD/BC numbering convention
///    observed.
///
/// 3) See also the function iauJd2cal.
///
/// 4) The number of decimal places ndp should be 4 or less if internal
///    overflows are to be avoided on platforms which use 16-bit
///    integers.
///
/// # Reference:
/// Explanatory Supplement to the Astronomical Almanac,
/// P. Kenneth Seidelmann (ed), University Science Books (1992),
/// Section 12.92 (p604).
pub fn jdcalf(ndp: i32, dj1: f64, dj2: f64) -> Result<(i32, i32, i32, i32, i32), i32> {
    let j;
    let denom;

    /* Denominator of fraction (e.g. 100 for 2 decimal places). */
    if (ndp >= 0) && (ndp <= 9) {
        j = 0;
        denom = 10.0f64.powi(ndp);
    } else {
        j = 1;
        denom = 1.0;
    }

    /* Copy the date, big then small. */
    let (mut d1, d2) = if dj1.abs() >= dj2.abs() {
        (dj1, dj2)
    } else {
        (dj2, dj1)
    };

    /* Realign to midnight (without rounding error). */
    d1 -= 0.5;

    /* Separate day and fraction (as precisely as possible). */
    let mut d = dnint(d1);
    let f1 = d1 - d;
    let mut djd = d;
    d = dnint(d2);
    let f2 = d2 - d;
    djd += d;
    d = dnint(f1 + f2);
    let mut f = (f1 - d) + f2;
    if f < 0.0 {
        f += 1.0;
        d -= 1.0;
    }
    djd += d;

    /* Round the total fraction to the specified number of places. */
    let rf = dnint(f * denom) / denom;

    /* Re-align to noon. */
    djd += 0.5;

    /* Convert to Gregorian calendar. */
    match jd2cal(djd, rf) {
        Ok((iy, im, id, f_out)) => {
            let iymdf_3 = dnint(f_out * denom) as i32;
            Ok((iy, im, id, iymdf_3, j))
        }
        Err(js) => Err(js),
    }
}

/* dnint(A) - round to nearest whole number (double) */
fn dnint(a: f64) -> f64 {
    if a.abs() < 0.5 {
        0.0
    } else if a < 0.0 {
        (a - 0.5).ceil()
    } else {
        (a + 0.5).floor()
    }
}
