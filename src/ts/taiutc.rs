use super::utctai;

///  Time scale transformation:  International Atomic Time, TAI, to
///  Coordinated Universal Time, UTC.
///
///  This function is part of the International Astronomical Union's
///  SOFA (Standards of Fundamental Astronomy) software collection.
///
///  Status:  canonical.
///
///  Given:
///     tai1,tai2  double   TAI as a 2-part Julian Date (Note 1)
///
///  Returned:
///     utc1,utc2  double   UTC as a 2-part quasi Julian Date (Notes 1-3)
///
///  Returned (function value):
///                int      status: +1 = dubious year (Note 4)
///                                  0 = OK
///                                 -1 = unacceptable date
///
///  Notes:
///
///  1) tai1+tai2 is Julian Date, apportioned in any convenient way
///     between the two arguments, for example where tai1 is the Julian
///     Day Number and tai2 is the fraction of a day.  The returned utc1
///     and utc2 form an analogous pair, except that a special convention
///     is used, to deal with the problem of leap seconds - see the next
///     note.
///
///  2) JD cannot unambiguously represent UTC during a leap second unless
///     special measures are taken.  The convention in the present
///     function is that the JD day represents UTC days whether the
///     length is 86399, 86400 or 86401 SI seconds.  In the 1960-1972 era
///     there were smaller jumps (in either direction) each time the
///     linear UTC(TAI) expression was changed, and these "mini-leaps"
///     are also included in the SOFA convention.
///
///  3) The function iauD2dtf can be used to transform the UTC quasi-JD
///     into calendar date and clock time, including UTC leap second
///     handling.
///
///  4) The warning status "dubious year" flags UTCs that predate the
///     introduction of the time scale or that are too far in the future
///     to be trusted.  See iauDat for further details.
///
///  Called:
///     iauUtctai    UTC to TAI
///
pub fn taiutc(tai1: f64, tai2: f64) -> Result<(f64, f64), i32> {
    let (a1, a2);
    let u1;
    let mut u2;
    let (mut g1, mut g2);

    /* Put the two parts of the TAI into big-first order. */
    let big1 = tai1.abs() >= tai2.abs();
    if big1 {
        a1 = tai1;
        a2 = tai2;
    } else {
        a1 = tai2;
        a2 = tai1;
    }

    /* Initial guess for UTC. */
    u1 = a1;
    u2 = a2;

    /* Iterate (though in most cases just once is enough). */
    for _ in 0..3 {
        /* Guessed UTC to TAI. */
        match utctai(u1, u2) {
            Ok((r1, r2)) => {
                g1 = r1;
                g2 = r2;
            }
            Err(status) => {
                return Err(status);
            }
        }

        /* Adjust guessed UTC. */
        u2 += a1 - g1;
        u2 += a2 - g2;
    }

    /* Return the UTC result, preserving the TAI order. */
    if big1 {
        Ok((u1, u2))
    } else {
        Ok((u2, u1))
    }
}
