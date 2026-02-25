use crate::cal::{cal2jd, jd2cal};
use crate::consts::DAYSEC;
use crate::ts::dat;

///  Time scale transformation:  Universal Time, UT1, to Coordinated
///  Universal Time, UTC.
///
///  This function is part of the International Astronomical Union's
///  SOFA (Standards of Fundamental Astronomy) software collection.
///
///  Status:  canonical.
///
///  Given:
///     ut11,ut12  double   UT1 as a 2-part Julian Date (Note 1)
///     dut1       double   Delta UT1: UT1-UTC in seconds (Note 2)
///
///  Returned:
///     utc1,utc2  double   UTC as a 2-part quasi Julian Date (Notes 3,4)
///
///  Returned (function value):
///                int      status: +1 = dubious year (Note 5)
///                                  0 = OK
///                                 -1 = unacceptable date
///
pub fn ut1utc(ut11: f64, ut12: f64, dut1: f64) -> Result<(f64, f64), i32> {
    let mut duts = dut1;
    let (u1, u2);

    /* Put the two parts of the UT1 into big-first order. */
    if ut11.abs() >= ut12.abs() {
        u1 = ut11;
        u2 = ut12;
    } else {
        u1 = ut12;
        u2 = ut11;
    }

    /* See if the UT1 can possibly be in a leap-second day. */
    let d1 = u1;
    let mut dats1 = 0.0;

    for i in -1..=3 {
        let d2 = u2 + (i as f64);
        let (iy, im, id, _) = match jd2cal(d1, d2) {
            Ok(v) => v,
            Err(_) => return Err(-1),
        };
        let dats2 = match dat(iy, im, id, 0.0) {
            Ok(v) => v,
            Err(_) => return Err(-1),
        };

        if i == -1 {
            dats1 = dats2;
        }
        let ddats = dats2 - dats1;
        if ddats.abs() >= 0.5 {
            /* Yes, leap second nearby: ensure UT1-UTC is "before" value. */
            if ddats * duts >= 0.0 {
                duts -= ddats;
            }

            /* UT1 for the start of the UTC day that ends in a leap. */
            let (us1, us2_base) = match cal2jd(iy, im, id) {
                Ok(v) => v,
                Err(_) => return Err(-1),
            };
            let us2 = us2_base - 1.0 + duts / DAYSEC;

            /* Is the UT1 after this point? */
            let mut du = u1 - us1;
            du += u2 - us2;
            if du > 0.0 {
                /* Yes:  fraction of the current UTC day that has elapsed. */
                let fd = du * DAYSEC / (DAYSEC + ddats);

                /* Ramp UT1-UTC to bring about SOFA's JD(UTC) convention. */
                duts += ddats * if fd <= 1.0 { fd } else { 1.0 };
            }

            /* Done. */
            break;
        }
        dats1 = dats2;
    }

    /* Subtract the (possibly adjusted) UT1-UTC from UT1 to give UTC. */
    let v2 = u2 - duts / DAYSEC;

    /* Result, safeguarding precision. */
    if ut11.abs() >= ut12.abs() {
        Ok((u1, v2))
    } else {
        Ok((v2, u1))
    }
}
