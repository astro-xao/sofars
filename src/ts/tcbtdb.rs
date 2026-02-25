use crate::consts::{DJM0, DJM77, ELB, TDB0, TTMTAI, DAYSEC};

///  Time scale transformation:  Barycentric Coordinate Time, TCB, to
///  Barycentric Dynamical Time, TDB.
///
///  This function is part of the International Astronomical Union's
///  SOFA (Standards of Fundamental Astronomy) software collection.
///
///  Status:  canonical.
///
///  Given:
///     tcb1,tcb2  double    TCB as a 2-part Julian Date
///
///  Returned:
///     tdb1,tdb2  double    TDB as a 2-part Julian Date
///
///  Returned (function value):
///                int       status:  0 = OK
///
pub fn tcbtdb(tcb1: f64, tcb2: f64) -> Result<(f64, f64), i32> {
    /* 1977 Jan 1 00:00:32.184 TT, as two-part JD */
    const T77TD: f64 = DJM0 + DJM77;
    let t77tf: f64 = TTMTAI / DAYSEC;

    /* TDB (days) at TAI 1977 Jan 1.0 */
    let tdb0_val: f64 = TDB0 / DAYSEC;

    let d;
    let (tdb1, tdb2);

    /* Result, safeguarding precision. */
    if tcb1.abs() > tcb2.abs() {
        d = tcb1 - T77TD;
        tdb1 = tcb1;
        tdb2 = tcb2 + tdb0_val - (d + (tcb2 - t77tf)) * ELB;
    } else {
        d = tcb2 - T77TD;
        tdb1 = tcb1 + tdb0_val - (d + (tcb1 - t77tf)) * ELB;
        tdb2 = tcb2;
    }

    Ok((tdb1, tdb2))
}
