use crate::consts::{DJM0, DJM77, ELB, TDB0, TTMTAI, DAYSEC};

///  Time scale transformation:  Barycentric Dynamical Time, TDB, to
///  Barycentric Coordinate Time, TCB.
///
///  This function is part of the International Astronomical Union's
///  SOFA (Standards of Fundamental Astronomy) software collection.
///
///  Status:  canonical.
///
///  Given:
///     tdb1,tdb2  double    TDB as a 2-part Julian Date
///
///  Returned:
///     tcb1,tcb2  double    TCB as a 2-part Julian Date
///
///  Returned (function value):
///                int       status:  0 = OK
///
pub fn tdbtcb(tdb1: f64, tdb2: f64) -> Result<(f64, f64), i32> {
    /* 1977 Jan 1 00:00:32.184 TT, as two-part JD */
    const T77TD: f64 = DJM0 + DJM77;
    let t77tf: f64 = TTMTAI / DAYSEC;

    /* TDB (days) at TAI 1977 Jan 1.0 */
    let tdb0_val: f64 = TDB0 / DAYSEC;

    /* TDB to TCB rate */
    let elbb: f64 = ELB / (1.0 - ELB);

    let (d, f);
    let (tcb1, tcb2);

    /* Result, preserving date format but safeguarding precision. */
    if tdb1.abs() > tdb2.abs() {
        d = T77TD - tdb1;
        f = tdb2 - tdb0_val;
        tcb1 = tdb1;
        tcb2 = f - (d - (f - t77tf)) * elbb;
    } else {
        d = T77TD - tdb2;
        f = tdb1 - tdb0_val;
        tcb1 = f - (d - (f - t77tf)) * elbb;
        tcb2 = tdb2;
    }

    Ok((tcb1, tcb2))
}
